extern crate key_list;

use key_list::KeyList;


#[test]
fn it_builds() {
    let text = "/a/ /b/ /c/";
    let _ = KeyList::new(text, '/', '/');
}

#[test]
fn it_parses() {
    let text = "/a/ /b/ /c/";
    let list = KeyList::new(text, '/', '/');

    assert_eq!(3, list.count());
}