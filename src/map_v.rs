use crate::utils::print_section_header;

/// along the real value transformation
pub(crate) fn map_v() {
    let fn_name = std::any::type_name_of_val(&map_v);
    print_section_header(fn_name);

    let o = Some("hello");
    let n = o.map(|x| x.chars().count()).map(|x| x.isqrt());
    println!("{:#?}", n);
    let r = n.ok_or("option to result error");
    println!("{:#?}", r);

    print_section_header(&format!("{} end", fn_name));
}
