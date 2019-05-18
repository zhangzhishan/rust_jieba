use std::collections::hash_map::HashMap;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use regex::Regex;


fn calc(sentence: &String, dag: BTreeMap<usize, Vec<usize>>) -> Vec<(f64, usize)> {
    let (freq, ltotal) = gen_pfdict().unwrap();
    // print!("{}", sentence);
    // let mut result:Vec<String> = Vec::new();
    let logtotal = (ltotal as f64).ln();
    let word_count = sentence.chars().count();
    let mut route: Vec<(f64, usize)> = Vec::new();
    route.resize(word_count + 1, (0_f64, 0));
    for idx in (0..word_count).rev() {
        println!("\n{}", idx);
        let mut flag = true;
        for x in dag.get(&idx).unwrap() {
            let temp;
            let mut temp_str: String = "".to_string();
            for i in idx..(x+1) {
                temp_str.push(sentence.chars().nth(i).unwrap());
            }
            if freq.contains_key(&temp_str) && (freq.get(&temp_str)).is_some() && *freq.get(&temp_str).unwrap() != 0 {
                temp = ((*freq.get(&temp_str).unwrap() as f64).ln() - logtotal + route[x + 1].0, *x);
            }
            else {
                temp = (route[x + 1].0 - logtotal, *x);
            }
            if flag {
                route[idx] = temp;
                flag = false;
            }
            if (route[idx].0 < temp.0) || (route[idx].0 == temp.0 && route[idx].1 < temp.1) {
                route[idx] = temp;
            }
        }
        println!("\nroute = ({}, {})", route[idx].0, route[idx].1)
    }
    route
}

pub fn cut_dag_no_hmm(sentence: String) -> Vec<String>{
    let re_eng = Regex::new(r"[a-zA-Z0-9]").unwrap();
    let mut result: Vec<String> = Vec::new();
    let (freq, _) = gen_pfdict().unwrap();
    // print!("{}", sentence);
    let dag = get_dag(freq, &sentence);
    let route = calc(&sentence, dag);
    let mut x:usize = 0;
    let word_count = sentence.chars().count();
    let mut buf = "".to_string();
    while x < word_count {
        let y = route[x].1 + 1;
        let mut l_word: String = "".to_string();
        for idx in x..y {
            l_word.push(sentence.chars().nth(idx).unwrap());
        }
        if re_eng.is_match(&l_word) && l_word.chars().count() == 1 {
            buf.push_str(&l_word);
        }
        else {
            if buf.chars().count() > 0 {
                result.push(buf);
                buf = "".to_string();
            }
            result.push(l_word);
        }
        x = y;
    }
    if buf.chars().count() > 0 {
        result.push(buf);
        buf = "".to_string();
    }
    result
}

pub fn cut(sentence: String) -> Vec<String>{
    let re_han_default = Regex::new(r"([\u4E00-\u9FD5a-zA-Z0-9+#&\._%\-]+)").unwrap();
    let re_skip_default = Regex::new(r"(\r\n|\s)").unwrap();
    // assert!(re_han_cut_all.is_match("张"));
    let mut result: Vec<String> = Vec::new();
    // let blocks = re_han.split(sentence)
    for blk in re_han_default.captures_iter(&sentence) {
        if re_han_default.is_match(&blk[0]) {
            for word in cut_dag_no_hmm((&blk[0]).to_string()) {
                result.push(word);
            }
        }
        else {
            for x in re_skip_default.captures_iter(&blk[0]) {
                if re_skip_default.is_match(&x[0]) {
                    result.push((&x[0]).to_string());
                }
                // else if not cut_all {
                //     for xx in x {
                //         result.push(xx);
                //     }
                // }
                else {
                    result.push((&x[0]).to_string());
                }
            }
        }
    }
    result
}

pub fn cut_all(sentence: String) -> Vec<String>{
    let re_han_cut_all = Regex::new(r"([\u4E00-\u9FD5]+)").unwrap();
    let re_skip_cut_all = Regex::new("[^a-zA-Z0-9+#\n]").unwrap();
    // assert!(re_han_cut_all.is_match("张"));
    let mut result: Vec<String> = Vec::new();
    // let blocks = re_han.split(sentence)
    for blk in re_han_cut_all.captures_iter(&sentence) {
        if re_han_cut_all.is_match(&blk[0]) {
            for word in cut_all_internal((&blk[0]).to_string()) {
                result.push(word);
            }
        }
        else {
            for x in re_skip_cut_all.captures_iter(&blk[0]) {
                if re_skip_cut_all.is_match(&x[0]) {
                    result.push((&x[0]).to_string());
                }
                // else if not cut_all {
                //     for xx in x {
                //         result.push(xx);
                //     }
                // }
                else {
                    result.push((&x[0]).to_string());
                }
            }
        }
    }
    result
}

pub fn cut_all_internal(sentence: String) -> Vec<String>{
    let (freq, _) = gen_pfdict().unwrap();
    // print!("{}", sentence);
    let mut result:Vec<String> = Vec::new();
    let dag = get_dag(freq, &sentence);
    // println!("dag len: {}", dag.len());
    let mut old_j:i32 = -1;
    for (k, l) in dag.iter() {
        // println!("l.len: {}", l.len());
        let temp_k = *k as i32;
        if l.len() == 1 && temp_k > old_j {
            old_j = l[0] as i32; //l[0];
            let mut temp_string: String = "".to_string();
            for idx in *k..(l[0] + 1) {
                temp_string.push(sentence.chars().nth(idx).unwrap());
            }
            // println!("tempString1: {}", temp_string);
            result.push(temp_string);
        }
        else {
            for j in l {
                if *j > *k {
                    old_j = *j as i32;
                    let mut temp_string: String = "".to_string();
                    for idx in *k..(*j + 1) {
                        temp_string.push(sentence.chars().nth(idx).unwrap());
                    }
                    // println!("tempString2: {}", temp_string);
                    result.push(temp_string);
                }
            }
        }
    }
    result

}

fn gen_pfdict() -> Result<(HashMap<String, i32>, i32)> {
    let mut lfreq = HashMap::new();
    let mut ltotal = 0;
    let file = File::open("C:\\Users\\zhizha.FAREAST\\Dropbox (Personal)\\code\\rustjieba\\rust_jieba\\extra_dict\\dict.txt.small")?;
    for line in BufReader::new(file).lines() {
        // println!("{}", line?);
        let oneline = line.unwrap();
        let v: Vec<&str> = oneline.split(' ').collect();
        let word = v[0].to_string();
        let freq = v[1].parse::<i32>().unwrap();
        lfreq.insert(word.clone(), freq);
        ltotal += freq;
        let mut wfrag = String::new();
        for c in word.chars() {
            wfrag.push(c);
            if !lfreq.contains_key(&wfrag) {
                lfreq.insert(wfrag.clone(), 0);
            }
        }
    }
    // for (key, val) in lfreq.iter() {
    //     println!("key: {} val: {}", key, val);
    // }

    Ok((lfreq, ltotal))
}


fn get_dag(freq: HashMap<String, i32>, sentence: &String) -> BTreeMap<usize, Vec<usize>> {
    let n = sentence.chars().count();
    let mut dag: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for k in 0..n {
        let mut tmplist: Vec<usize> = Vec::new();
        let mut i = k;
        let mut frag = sentence.chars().nth(k).unwrap().to_string();
        // println!("{}", &frag);
        
        while i < n && freq.contains_key(&frag) {
            if (freq.get(&frag)).is_some() && *freq.get(&frag).unwrap() != 0 {
                // println!("frag: {}, value: {}", &frag, *freq.get(&frag).unwrap());
                tmplist.push(i);
            }

            i += 1;
            if i >= n {
                continue;
            }
            frag = "".to_string();
            for idx in k..(i + 1) {
                // print!(" idx: {}", idx);
                frag.push(sentence.chars().nth(idx).unwrap());
            }
        }

        if tmplist.is_empty() {
            tmplist.push(k);
        }
        // println!("k: {}", k);
        // for idx in &tmplist {
        //     print!(" templist: {} ", idx);

        // }
        dag.insert(k, tmplist);
    }
    return dag;
}

