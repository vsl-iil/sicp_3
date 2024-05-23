trait Readable {
    fn read(&self) -> String;

    fn read_twice(&self) -> Vec<String> {
        vec![
            self.read(),
            self.read()
        ]
    }

    fn annotate(&mut self, text: &str);
}

#[allow(dead_code)]
struct Book {
    page_amount: usize,
    contents:    String,
}

struct Newspaper {
    contents: String,
    has_crosswords: bool,
}

struct Blogpost {
    user: String,
    text: String,
    comments: Vec<String>,
}


impl Readable for Book {
    fn read(&self) -> String {
        self.contents.clone()
    }

    fn annotate(&mut self, text: &str) {
        self.contents.push_str(text)
    }
}

impl Readable for Newspaper {
    fn read(&self) -> String {
        // Если кроссвордов нет, то 
        // не стоит и начинать читать
        if !self.has_crosswords {
            String::from("Нет кроссвордов :(")
        } else {
            self.contents.clone()
        }
    }

    fn annotate(&mut self, text: &str) {
        self.contents.push_str(text)
    }
}

impl Readable for Blogpost {
    fn read(&self) -> String {
        format!("{} написал: {}", self.user, self.text)
    }

    fn read_twice(&self) -> Vec<String> {
        let mut out = vec![self.read()];
        out.append(&mut self.comments.clone());

        out
    }

    fn annotate(&mut self, text: &str) {
        self.comments.push(format!("\tКомментарий: {}", text.to_owned()))
    }
}


fn main() {

    println!("\n\t***\n");

    let mut lord_of_the_rings = Book {
        page_amount: 1077,
        contents: "Когда Бильбо Бэггинс, владелец Торбы-на-Круче, объявил...".to_string(),
    };

    let pravda = Newspaper {
        contents: "Последние новости: ...".to_string(),
        has_crosswords: false,
    };

    let mut weblog = Blogpost {
        user: "Кори Доктороу".to_string(),
        text: "Here is how platforms die: ...".to_string(),
        comments: vec![],
    };

    println!("{}", lord_of_the_rings.read());
    let read_vec = lord_of_the_rings.read_twice();
    println!("{}; прочтём ещё раз: {}", read_vec[0], read_vec[1]);
    lord_of_the_rings.annotate("Отличная книга!");

    println!("\n\t***\n");

    println!("{}", pravda.read());

    println!("\n\t***\n");

    println!("{}", weblog.read());
    weblog.annotate("отличная статья!");
    let read_vec = weblog.read_twice();
    println!("{} {}", read_vec[0], read_vec[1]);
}
