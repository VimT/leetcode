//! 标签验证器

// 代码必须被合法的闭合标签包围。否则，代码是无效的。
// 闭合标签（不一定合法）要严格符合格式：<TAG_NAME>TAG_CONTENT</TAG_NAME>。其中，<TAG_NAME>是起始标签，</TAG_NAME>是结束标签。起始和结束标签中的 TAG_NAME 应当相同。当且仅当 TAG_NAME 和 TAG_CONTENT 都是合法的，闭合标签才是合法的。
// 合法的 TAG_NAME 仅含有大写字母，长度在范围 [1,9] 之间。否则，该 TAG_NAME 是不合法的。
// 合法的 TAG_CONTENT 可以包含其他合法的闭合标签，cdata （请参考规则7）和任意字符（注意参考规则1）除了不匹配的<、不匹配的起始和结束标签、不匹配的或带有不合法 TAG_NAME 的闭合标签。否则，TAG_CONTENT 是不合法的。
// 一个起始标签，如果没有具有相同 TAG_NAME 的结束标签与之匹配，是不合法的。反之亦然。不过，你也需要考虑标签嵌套的问题。
// 一个<，如果你找不到一个后续的>与之匹配，是不合法的。并且当你找到一个<或</时，所有直到下一个>的前的字符，都应当被解析为 TAG_NAME（不一定合法）。
// cdata 有如下格式：<![CDATA[CDATA_CONTENT]]>。CDATA_CONTENT 的范围被定义成 <![CDATA[ 和后续的第一个 ]]>之间的字符。
// CDATA_CONTENT 可以包含任意字符。cdata 的功能是阻止验证器解析CDATA_CONTENT，所以即使其中有一些字符可以被解析为标签（无论合法还是不合法），也应该将它们视为常规字符。

struct XMLParse<'a> {
    i: usize,
    s: &'a [u8],
}

type Result<T> = std::result::Result<T, ()>;

impl<'a> XMLParse<'a> {
    fn new(s: &'a [u8]) -> Self {
        Self { i: 0, s }
    }

    #[inline]
    fn len(&self) -> usize { self.s.len() }

    #[inline]
    fn next(&mut self) -> Result<u8> {
        if self.i < self.len() {
            let ret = self.s[self.i];
            self.i += 1;
            return Ok(ret);
        }
        Err(())
    }

    #[inline]
    fn preview(&self) -> Result<u8> {
        if self.i < self.len() {
            return Ok(self.s[self.i]);
        }
        Err(())
    }

    #[inline]
    fn previewn(&self, n: usize) -> Result<&[u8]> {
        if self.i + n <= self.len() {
            return Ok(&self.s[self.i..self.i + n]);
        }
        Err(())
    }

    fn parse(&mut self) -> Result<()> {
        self.tag()?;
        if self.i != self.len() { return Err(()); }
        Ok(())
    }

    fn tag(&mut self) -> Result<()> {
        let mut tag_name = vec![];
        if self.next()? != b'<' { return Err(()); }
        while self.preview()? != b'>' {
            let tag_name_char = self.next()?;
            if !tag_name_char.is_ascii_uppercase() {
                return Err(());
            }
            tag_name.push(tag_name_char);
            if tag_name.len() > 9 { return Err(()); }
        }
        self.next()?;
        if tag_name.is_empty() { return Err(()); }
        self.content()?;
        if self.next()? != b'<' { return Err(()); }
        if self.next()? != b'/' { return Err(()); }
        for &ch in &tag_name {
            if self.next()? != ch {
                return Err(());
            }
        }
        if self.next()? != b'>' { return Err(()); }
        Ok(())
    }

    fn content(&mut self) -> Result<()> {
        loop {
            while self.preview()? != b'<' {
                self.next()?;
            }
            match self.previewn(2)?[1] {
                b'/' => { return Ok(()); }
                b'!' => self.cdata()?,
                _ => self.tag()?
            }
        }
    }

    fn cdata(&mut self) -> Result<()> {
        for &ch in b"<![CDATA[" {
            if self.next()? != ch {
                return Err(());
            }
        }
        loop {
            while self.preview()? != b']' {
                self.next()?;
            }
            if self.previewn(3)? == b"]]>" {
                for _ in 0..3 { self.next()?; }
                return Ok(());
            } else {
                self.next()?;
            }
        }
    }
}

pub fn is_valid(code: String) -> bool {
    let mut parser = XMLParse::new(code.as_bytes());
    parser.parse().is_ok()
}

fn main() {
    assert_eq!(is_valid(String::from("<A></A><B></B>")), false);
    assert_eq!(is_valid(String::from("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>")), true);
    assert_eq!(is_valid(String::from("<DIV>This is the first line <![CDATA[<div>]]></DIV>")), true);
    assert_eq!(is_valid(String::from("<A>  <B> </A>   </B>")), false);
    assert_eq!(is_valid(String::from("<DIV>  div tag is not closed  <DIV>")), false);
}
