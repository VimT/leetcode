import glob
import html
import json
import logging
import os
import re
import subprocess
import time

import click
import pyperclip
import requests

logging.basicConfig(format="[%(asctime)s: %(levelname)s] %(message)s", level=logging.INFO)
logger = logging.getLogger(__name__)

session = requests.session()
if os.path.exists('cookie'):
    with open('cookie', 'r', encoding='utf-8') as f:
        cookie = f.read()
        session.headers['cookie'] = cookie.strip()


def get_problem_detail(slug: str):
    url = "https://leetcode.cn/graphql"
    query = """
      query getQuestionDetail($titleSlug: String!) {
        question(titleSlug: $titleSlug) {
          content
          stats
          codeDefinition
          sampleTestCase
          enableRunCode
          metaData
          translatedTitle
          translatedContent
          questionFrontendId
        }
      }
    """
    payload = {
        "query": query,
        "variables": {"titleSlug": slug},
        "operationName": "getQuestionDetail"
    }
    rsp = session.post(url, json=payload)
    data = rsp.json()['data']['question']
    if not data['codeDefinition']:
        return None
    code = json.loads(data['codeDefinition'])
    return ProblemDetail(data['questionFrontendId'], data['translatedTitle'], data['content'],
                         {i['value']: i['defaultCode'] for i in code})


class Problem(object):
    def __init__(self, leetcode_data):
        self.id = leetcode_data['frontendQuestionId'].replace(' ', '_').lower()
        self.title = leetcode_data['title']
        self.key = leetcode_data['titleSlug']
        self.ch_title = leetcode_data['titleCn']


def trans_arg(arg: str, type: str, is_return: bool = False):
    arg = arg.strip()
    if '=' in arg:
        arg = arg.partition('=')[2].strip()
    if 'char' in type:
        arg = arg.replace('"', "'")
    if type.startswith("Vec"):
        if type == "Vec<String>" and not is_return:
            return f"svec!{arg}"
        elif type == "Vec<Vec<String>>":
            return f"vec![" + arg[1:-1].replace('[', 'svec![') + ']'
        elif type == "Vec<Option<Rc<RefCell<TreeNode>>>>":
            return 'vec![' + arg[1:-1].replace('[', 'tree![') + ']'
        elif type == "Vec<Option<Box<ListNode>>>":
            return 'vec![' + arg[1:-1].replace('[', 'link![') + ']'
        else:
            return arg.replace('[', 'vec![')
    elif type == "String":
        return f"String::from({arg})"
    elif type == "Option<Rc<RefCell<TreeNode>>>":
        return 'tree!' + arg
    elif type == "Option<Box<ListNode>>":
        return 'link!' + arg
    else:
        return arg


class ProblemDetail(object):
    def __init__(self, question_id, ch_title, content, templates):
        self.id = question_id.replace('面试题 ', 'm').replace('剑指 ', '').replace('.', '_').replace(' ', '_').lower()
        self.ch_title = ch_title
        self.content = content
        self.templates = templates
        self.unorder = 'any order' in content

    def rust_template(self, cases=''):
        code: str = self.templates['rust']
        lines = code.split('\n')
        lines = [i.strip() for i in lines if not i.strip().startswith('//')]
        main_use = []
        if 'impl Solution {' in lines:
            lines.remove('impl Solution {')
            lines.remove('}')
        if 'TreeNode' in code:
            lines.insert(0, "use leetcode::treenode::TreeNode;")
            main_use.append("use leetcode::tree;")
            lines.insert(2, "")
        if 'ListNode' in code:
            lines.insert(0, "use leetcode::linknode::ListNode;")
            main_use.append("use leetcode::link;")
            lines.insert(2, "")
        if 'svec![' in cases:
            main_use.append("use leetcode::svec;")
        if self.unorder:
            main_use.append('use leetcode::unorder;')
        return '\n'.join(lines), '\n'.join(main_use)

    def rust_testcase(self):
        code = self.templates['rust']
        funcs = [i for i in code.split('\n') if i.strip().startswith('pub fn')]
        if len(funcs) > 1:
            raise Exception("multi funcs")
        func: str = funcs[0]
        funcname, _, other = func.partition('(')
        args, _, return_ = other.partition(')')
        funcname = funcname.strip().removeprefix('pub fn').strip()
        args_type = [i.partition(':')[2].strip() for i in args.split(', ')]
        return_type = return_.partition('->')[2].strip().partition('{')[0].strip()
        content = self.content
        lines = content.split('\n')
        inputs = [i for i in lines if '<strong>Input:</strong>' in i]
        outputs = [i for i in lines if '<strong>Output:</strong>' in i]
        outputs += [''] * (len(inputs) - len(outputs))
        result = []
        cases = []
        for i, o in zip(inputs, outputs):
            i = html.unescape(i.replace('<strong>Input:</strong>', ''))
            o = html.unescape(o.replace('<strong>Output:</strong>', ''))
            args = i.split(', ')
            transed = [trans_arg(arg, type) for arg, type in zip(args, args_type)]
            cases.append((f"{','.join(transed)}", trans_arg(o, return_type, True)))
        if True:
            origin_func_args = '(' + func.partition('(')[2].partition('{')[0]
            result.append(f"fn test(func: fn{origin_func_args}) {{")
            for func_in, func_out in cases:
                if self.unorder:
                    result.append(f"assert_eq!(unorder(func({func_in})), unorder({func_out}));")
                else:
                    result.append(f"assert_eq!(func({func_in}), {func_out});")
            result.append("}")
            result.append(f"test({funcname});")
        # else:
        #     for func_in, func_out in cases:
        #         if unorder:
        #             result.append(f"assert_eq!(unorder({funcname}({func_in})), unorder({func_out}));")
        #         else:
        #             result.append(f"assert_eq!({funcname}({func_in}), {func_out});")
        return result


def get_problems(keyword='', skip=0, limit=50):
    url = "https://leetcode.cn/graphql"
    query = """
    query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
      problemsetQuestionList(
        categorySlug: $categorySlug
        limit: $limit
        skip: $skip
        filters: $filters
      ) {
        hasMore
        total
        questions {
          acRate
          difficulty
          freqBar
          frontendQuestionId
          isFavor
          paidOnly
          solutionNum
          status
          title
          titleCn
          titleSlug
        }
      }
    }

    """
    payload = {
        "query": query,
        "variables": {"categorySlug": "algorithms", "skip": skip, "limit": limit, "filters": {}}
    }
    if keyword:
        payload['variables']['filters']["searchKeywords"] = str(keyword)
    rsp = session.post(url, json=payload)
    data = rsp.json()['data']['problemsetQuestionList']['questions']
    problem = [Problem(i) for i in data]
    return {i.id: i for i in problem}


def contest_problems(name: str):
    """return list of
        category_slug: "contest"
        credit: 3
        english_title: "Categorize Box According to Criteria"
        id: 4658
        question_id: 2619
        title: "根据规则将箱子分类"
        title_slug: "categorize-box-according-to-criteria"
    """
    url = f'https://leetcode.cn/contest/api/info/{name}'
    data = session.get(url).json()
    return data['questions']


def contest_problem_detail(name: str, title_slug: str):
    url = f'https://leetcode.cn/contest/{name}/problems/{title_slug}/'
    rsp = session.get(url, allow_redirects=False)
    if rsp.status_code != 200:
        raise Exception(f"problem detail response code [{rsp.status_code}] != 200: {rsp.text}")
    lines = rsp.text.split('\n')
    pid = [i for i in lines if '<h3>' in i][0].strip().removeprefix('<h3>').partition('.')[0]
    title = [i for i in lines if 'questionTitle: ' in i][0].strip().removeprefix('questionTitle: ').strip("',")
    codes_str = [i for i in lines if 'codeDefinition: ' in i][0].strip().removeprefix('codeDefinition: ').strip(',').replace("'", '"')[:-2] + "]"
    codes = json.loads(codes_str)
    content = [i for i in lines if 'questionSourceContent: ' in i][0].strip().removeprefix(
        'questionSourceContent: ').strip("',").encode('utf-8').decode('unicode_escape')

    return ProblemDetail(pid, title, content, {i['value']: i['defaultCode'] for i in codes})


BASE_DIR = os.path.dirname(os.path.realpath(__file__))


@click.group()
def cli():
    """generate leetcode rust problem file"""


def write(filepath, detail: ProblemDetail):
    with open(filepath, "w", encoding='utf-8') as f:
        cases = ""
        try:
            cases = detail.rust_testcase()
        except Exception as e:
            logger.error(f"generate testcases fail: {e}")
        f.write(f"//! {detail.ch_title}\n")
        f.write("\n")
        code, main_use = detail.rust_template('\n'.join(cases))
        f.write(code)
        f.write("\n")
        f.write("\n")
        f.write("fn main() {\n")
        f.write(main_use)
        f.write('\n'.join(cases))
        f.write('\n')
        f.write('}\n')
    print(filepath)
    subprocess.run(f'git add {filepath}', shell=True)
    time.sleep(1)


@cli.command()
@click.option('-f', '--force', is_flag=True)
@click.option('-s', '--slug', is_flag=True)
@click.argument('pids', nargs=-1)
def get(pids, force, slug):
    for pid in pids:
        pid = pid.strip()
        path = ''
        if slug:
            slug = pid
        else:
            path = os.path.join(BASE_DIR, "src", "bin", f"leetcode_{pid}.rs")
            if os.path.exists(path):
                if not force:
                    logger.error(f"path {path} exist")
                    continue
                else:
                    logger.warning(f"will replace {path}")
            problems = get_problems(pid)
            problem = problems[str(pid)]
            slug = problem.key
        detail = get_problem_detail(slug)
        if not detail:
            logger.warning("get empty problem detail")
            continue
        if path == '':
            path = os.path.join(BASE_DIR, "src", "bin", f"leetcode_{detail.id}.rs")
            if os.path.exists(path):
                if not force:
                    logger.error(f"path {path} exist")
                    return
                else:
                    logger.warning(f"will replace {path}")
        write(path, detail)


@cli.command('range')
@click.argument('start')
@click.argument('end')
def range_(start, end):
    start, end = int(start), int(end)
    problems = get_problems(skip=start - 1, limit=100)
    filtered = [int(id) for id in problems if id.isnumeric() and start <= int(id) <= end]
    for id in filtered:
        try:
            get.callback(str(id), False, True, '', False)
        except Exception as e:
            logger.error(f"get {id} fail: {e}")


@cli.command()
def fix_id():
    for file in glob.glob("src/bin/leetcode_*.rs"):
        pid = file.partition('_')[2].partition('.')[0]
        if not pid.isdigit():
            continue
        pid = int(pid)
        if pid < 5000:
            continue
        with open(file, 'r', encoding='utf-8') as f:
            first_line = f.readline()
            ch_title = first_line.strip('/! ').strip()
        problems = get_problems(ch_title)
        problems = [v for k, v in problems.items() if v.ch_title == ch_title]
        if problems:
            problem = problems[0]
            real_id = int(problem.id)
            if real_id != pid:
                to = file.replace(str(pid), str(real_id))
                print(f"rename {file} -> {to}")
                os.rename(file, to)
                subprocess.run(f'git add {to}', shell=True)


@cli.command()
@click.argument('name')
def contest(name):
    for question in contest_problems(name):
        detail = contest_problem_detail(name, question['title_slug'])
        path = os.path.join(BASE_DIR, "src", "bin", f"leetcode_{detail.id}.rs")
        if os.path.exists(path):
            logger.error(f"path {path} exist")
            continue
        write(path, detail)


@cli.command()
@click.option('-f', '--func', 'wanted_func', help="commit function name")
@click.argument('filename')
def copy(filename: str, wanted_func):
    if filename.isdigit():
        filename = f'src/bin/leetcode_{filename}.rs'
    with open(filename) as f:
        content = f.read()
    content, _, main = content.partition('fn main(')
    if '::new' in main:
        pyperclip.copy('\n'.join([i for i in content.split('\n') if not i.startswith('//!')]))
        return
    problem_func_name = main.partition('    test(')[2].partition(')')[0]
    if not problem_func_name:
        print("unknown problem func name")
        return
    use = []
    problem_funcs = {}
    other_funcs = {}
    in_func = False
    cur_func = []
    other = []
    for line in content.split('\n'):
        if in_func:
            cur_func.append(line)
            if line == '}':
                func = '\n'.join(cur_func)
                func_name = func.partition('(')[0].partition('fn ')[2]
                if func_name.startswith(problem_func_name):
                    problem_funcs[func_name] = func
                else:
                    other_funcs[func_name] = func
                cur_func = []
                in_func = False
        elif line and not line.startswith('//!'):
            if line.startswith('use '):
                use.append(line)
            elif line.startswith('pub fn ') or line.startswith('fn '):
                in_func = True
                while other and other[-1].startswith('///'):
                    cur_func.append(other.pop())
                cur_func.append(line)
            else:
                other.append(line)

    clip = pyperclip.paste()
    if not wanted_func and clip in problem_funcs:
        wanted_func = clip
    if wanted_func:
        func_name, func_body = [(k, v) for k, v in problem_funcs.items() if k.endswith(wanted_func)][0]
    else:
        func_name, func_body = problem_func_name, problem_funcs[problem_func_name]
    result = []
    for u in use:
        if 'leetcode::' in u:
            print(f"please handle {u}")
            continue
        prefix, _, names = u.rpartition('::')
        names = [i.strip() for i in names.strip(';{}').split(',')]
        used_names = [i for i in names if i in func_body]
        if used_names:
            used_names = used_names[0] if len(used_names) == 1 else '{' + ', '.join(used_names) + '}'
            result.append(f'{prefix}::{used_names};')
    other_funcs_content = []
    for other_func_name, other_func_body in other_funcs.items():
        if other_func_name in func_body:
            other_funcs_content.append(other_func_body)
    other_funcs_content = '\n\n'.join(other_funcs_content)
    for o in other:
        if o.startswith('static ') or o.startswith('const '):
            result.append(o)
    if other_funcs_content:
        result.append(other_funcs_content)
    func_body = func_body.replace(func_name, problem_func_name)
    result.append('impl Solution {')
    result.extend(['    ' + i for i in func_body.split('\n')])
    result.append('}')
    pyperclip.copy('\n'.join(result))


if __name__ == '__main__':
    cli()
