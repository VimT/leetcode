//! baby-names-lcci


use std::cmp::Ordering;
use std::collections::HashMap;

struct UnionSet {
    f: Vec<usize>,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        UnionSet {
            f: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x {
            x
        } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut xx = self.find(x);
        let mut yy = self.find(y);
        self.f[yy] = xx;
    }
}


pub fn truly_most_popular(names: Vec<String>, synonyms: Vec<String>) -> Vec<String> {
    let len = names.len();
    let max_len = synonyms.len() * 2;
    let mut name_idx_map = HashMap::with_capacity(max_len);
    let mut name_frq_map = HashMap::with_capacity(max_len);
    let mut ns = Vec::with_capacity(max_len);
    for (idx, name_frq) in names.iter().enumerate() {
        let sp = name_frq.find("(").unwrap();
        let name = name_frq[..sp].to_string();
        let frq = name_frq[sp + 1..name_frq.len() - 1].parse::<i64>().unwrap();
        name_idx_map.insert(name.clone(), idx);
        name_frq_map.insert(name.clone(), frq);
        ns.push(name);
    }

    for synonym in synonyms.iter() {
        let sp = synonym.find(",").unwrap();
        let name1 = synonym[1..sp].to_string();
        let name2 = synonym[sp + 1..synonym.len() - 1].to_string();
        if !name_idx_map.contains_key(&name1) {
            name_idx_map.insert(name1.clone(), ns.len());
            name_frq_map.insert(name1.clone(), 0);
            ns.push(name1.clone());
        }
        if !name_idx_map.contains_key(&name2) {
            name_idx_map.insert(name2.clone(), ns.len());
            name_frq_map.insert(name2.clone(), 0);
            ns.push(name2.clone());
        }
    }
    let mut us = UnionSet::new(ns.len());

    for synonym in synonyms {
        let sp = synonym.find(",").unwrap();
        let name1 = synonym[1..sp].to_string();
        let name2 = synonym[sp + 1..synonym.len() - 1].to_string();

        let idx1 = *name_idx_map.get(&name1).unwrap();
        let idx2 = *name_idx_map.get(&name2).unwrap();

        let parent1 = us.find(idx1);
        let parent2 = us.find(idx2);
        let mut min_idx = idx1;
        if name2 < ns[min_idx] { min_idx = idx2; }
        if ns[parent1] < ns[min_idx] { min_idx = parent1; }
        if ns[parent2] < ns[min_idx] { min_idx = parent2; }
        us.union(min_idx, idx1);
        us.union(min_idx, idx2);
    }
    let mut ans = vec![0; len];
    for (idx, name) in ns.iter().enumerate() {
        let frq = name_frq_map.get(name).unwrap();
        let parent = us.find(idx);
        ans[parent] += frq;
    }
    let mut real_ans = vec![];
    for (idx, frq) in ans.into_iter().enumerate() {
        if frq > 0 {
            real_ans.push(format!("{}({})", ns[idx], frq));
        }
    }
    real_ans
}

fn main() {
    println!("{:?}", truly_most_popular(
        vec!["Fcclu(70)", "Ommjh(63)", "Dnsay(60)", "Qbmk(45)", "Unsb(26)", "Gauuk(75)", "Wzyyim(34)", "Bnea(55)", "Kri(71)", "Qnaakk(76)", "Gnplfi(68)", "Hfp(97)", "Qoi(70)", "Ijveol(46)", "Iidh(64)", "Qiy(26)", "Mcnef(59)", "Hvueqc(91)", "Obcbxb(54)", "Dhe(79)", "Jfq(26)", "Uwjsu(41)", "Wfmspz(39)", "Ebov(96)", "Ofl(72)", "Uvkdpn(71)", "Avcp(41)", "Msyr(9)", "Pgfpma(95)", "Vbp(89)", "Koaak(53)", "Qyqifg(85)", "Dwayf(97)", "Oltadg(95)", "Mwwvj(70)", "Uxf(74)", "Qvjp(6)", "Grqrg(81)", "Naf(3)", "Xjjol(62)", "Ibink(32)", "Qxabri(41)", "Ucqh(51)", "Mtz(72)", "Aeax(82)", "Kxutz(5)", "Qweye(15)", "Ard(82)", "Chycnm(4)", "Hcvcgc(97)", "Knpuq(61)", "Yeekgc(11)", "Ntfr(70)", "Lucf(62)", "Uhsg(23)", "Csh(39)", "Txixz(87)", "Kgabb(80)", "Weusps(79)", "Nuq(61)", "Drzsnw(87)", "Xxmsn(98)", "Onnev(77)", "Owh(64)", "Fpaf(46)", "Hvia(6)", "Kufa(95)", "Chhmx(66)", "Avmzs(39)", "Okwuq(96)", "Hrschk(30)", "Ffwni(67)", "Wpagta(25)", "Npilye(14)", "Axwtno(57)", "Qxkjt(31)", "Dwifi(51)", "Kasgmw(95)", "Vgxj(11)", "Nsgbth(26)", "Nzaz(51)", "Owk(87)", "Yjc(94)", "Hljt(21)", "Jvqg(47)", "Alrksy(69)", "Tlv(95)", "Acohsf(86)", "Qejo(60)", "Gbclj(20)", "Nekuam(17)", "Meutux(64)", "Tuvzkd(85)", "Fvkhz(98)", "Rngl(12)", "Gbkq(77)", "Uzgx(65)", "Ghc(15)", "Qsc(48)", "Siv(47)"].iter().map(|x| x.to_string()).collect(),
        vec!["(Gnplfi,Qxabri)", "(Uzgx,Siv)", "(Bnea,Lucf)", "(Qnaakk,Msyr)", "(Grqrg,Gbclj)", "(Uhsg,Qejo)", "(Csh,Wpagta)", "(Xjjol,Lucf)", "(Qoi,Obcbxb)", "(Npilye,Vgxj)", "(Aeax,Ghc)", "(Txixz,Ffwni)", "(Qweye,Qsc)", "(Kri,Tuvzkd)", "(Ommjh,Vbp)", "(Pgfpma,Xxmsn)", "(Uhsg,Csh)", "(Qvjp,Kxutz)", "(Qxkjt,Tlv)", "(Wfmspz,Owk)", "(Dwayf,Chycnm)", "(Iidh,Qvjp)", "(Dnsay,Rngl)", "(Qweye,Tlv)", "(Wzyyim,Kxutz)", "(Hvueqc,Qejo)", "(Tlv,Ghc)", "(Hvia,Fvkhz)", "(Msyr,Owk)", "(Hrschk,Hljt)", "(Owh,Gbclj)", "(Dwifi,Uzgx)", "(Iidh,Fpaf)", "(Iidh,Meutux)", "(Txixz,Ghc)", "(Gbclj,Qsc)", "(Kgabb,Tuvzkd)", "(Uwjsu,Grqrg)", "(Vbp,Dwayf)", "(Xxmsn,Chhmx)", "(Uxf,Uzgx)"].iter().map(|x| x.to_string()).collect(),
    ));
    println!("{:?}", truly_most_popular(
        vec!["John(15)", "Jon(12)", "Chris(13)", "Kris(4)", "Christopher(19)"].iter().map(|x| x.to_string()).collect(),
        vec!["(Jon,John)", "(John,Johnny)", "(Chris,Kris)", "(Chris,Christopher)"].iter().map(|x| x.to_string()).collect()));
    println!("{:?}", truly_most_popular(
        ["a(10)", "c(13)"].iter().map(|x| x.to_string()).collect(),
        ["(a,b)", "(c,d)", "(b,c)"].iter().map(|x| x.to_string()).collect()));
}
