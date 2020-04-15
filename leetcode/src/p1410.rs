// https://leetcode-cn.com/problems/html-entity-parser/

pub fn entity_parser(text: String) -> String {
    let mut i = 0;
    let l = text.len();
    let mut res = String::new();

    while i < l {
        if &text[i..=i] != "&" {
            res.push_str(&text[i..=i]);
            i += 1;
        } else {
            if i + 5 < l && &text[i..=i + 5] == "&quot;" {
                res.push_str("\"");
                i += 6;
            } else if i + 5 < l && &text[i..=i + 5] == "&apos;" {
                res.push_str("'");
                i += 6;
            } else if i + 4 < l && &text[i..=i + 4] == "&amp;" {
                res.push_str("&");
                i += 5;
            } else if i + 3 < l && &text[i..=i + 3] == "&gt;" {
                res.push_str(">");
                i += 4;
            } else if i + 3 < l && &text[i..=i + 3] == "&lt;" {
                res.push_str("<");
                i += 4;
            } else if i + 6 < l && &text[i..=i + 6] == "&frasl;" {
                res.push_str("/");
                i += 7;
            } else {
                res.push_str("&");
                i += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1410() {
        assert_eq!(
            "& is an HTML entity but &ambassador; is not.",
            entity_parser(String::from(
                "&amp; is an HTML entity but &ambassador; is not."
            ))
        );
        assert_eq!(
            "and I quote: \"...\"",
            entity_parser(String::from("and I quote: &quot;...&quot;"))
        );
        assert_eq!(
            "Stay home! Practice on Leetcode :)",
            entity_parser(String::from("Stay home! Practice on Leetcode :)"))
        );
        assert_eq!(
            "x > y && x < y is always false",
            entity_parser(String::from("x &gt; y &amp;&amp; x &lt; y is always false"))
        );
        assert_eq!(
            "leetcode.com/problemset/all",
            entity_parser(String::from("leetcode.com&frasl;problemset&frasl;all"))
        );
        assert_eq!("\" > >", entity_parser(String::from("&quot; &gt; &gt;")));
    }
}
