#[cfg(test)]
mod tests {
    #[test]
    fn test_syn() {
        let src = r#"
            *s = S {
                a: 1,
                b: _,
                c,
            };
        "#;
        let res = syn::parse_str::<syn::Stmt>(src).unwrap();
        println!("{:#?}", res);
    }

    #[test]
    fn test_self_assign() {
        struct S {
            a: u64,
            b: String,
            c: String,
        }

        let mut s = S {
            a: 0,
            b: "test".into(),
            c: "foo".into(),
        };

        let c = "bar".to_string();

        self_assign::self_assign! {
            s = S {
                a: 1,
                b: _,
                c,
            };
        }

        assert_eq!(s.a, 1);
        assert_eq!(s.b, "test");
        assert_eq!(s.c, "bar");
    }
}
