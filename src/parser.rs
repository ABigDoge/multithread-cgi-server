
extern crate httparse;
use httparse::Request;

#[derive(Debug)]
pub struct Loginfo {
    method : String,
    host : String,
    user : String,
    path : String,
    length : usize,
    iscgi : bool
}

#[derive(Debug)]
pub struct Pkg<'headers, 'buf> {
    pub log : Loginfo,
    pub req : Request<'headers, 'buf>
}

pub fn parser (s : String) -> Pkg<'static, 'static> {

    let le = s.len();
    let su = &s.as_bytes();
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers[..]);
    let res = req.parse(su).unwrap();

    let mut iscgi = false;
    let partpath: String = req.path.unwrap().chars().take(8).collect();
    if partpath == "/cgi-bin" {
        iscgi = true;
    }

    let mut host = String::new();
    let mut user = String::new();

    let mut index = 0;
    while index < 10 {
        let i = req.headers[index];
        if i.name.to_string() == "Host" {
            host = String::from_utf8(i.value.to_vec()).unwrap();
        }

        if i.name.to_string() == "User-Agent" {
            user = String::from_utf8(i.value.to_vec()).unwrap();
        }

        index = index + 1;
    }


    let log = Loginfo {
        method: req.method.unwrap().to_string(),
        host: host,
        user: user,
        path: req.path.unwrap().to_string(),
        length: le,
        iscgi: iscgi,
    };
    
    Pkg {
            log : log,
            req : req
        }
}