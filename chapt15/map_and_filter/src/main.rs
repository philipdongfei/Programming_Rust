fn main() {
    {
        let text = "  ponies \n giraffes\niguanas \nsquid".to_string();
        let v: Vec<&str> = text.lines()
            .map(str::trim)
            .collect();
        assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);

    }
    {
        let text = "  ponies \n giraffes\niguanas \nsquid".to_string();
        let v: Vec<&str> = text.lines()
            .map(str::trim)
            .filter(|s| *s != "iguanas")
            .collect();
        assert_eq!(v, ["ponies", "giraffes", "squid"]);


    }
}

