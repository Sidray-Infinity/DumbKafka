use std::any;
use std::fs;
use std::io::BufReader;
use std::io::Write;
use std::os::macos::raw;
use std::path::Path;
use std::str;


// produced by the producer
#[derive(Copy, Clone, Debug)]
struct Message<'a> {
    raw_data: &'a [u8],
}

struct Writer {}

impl Writer {
    fn write(&self, topic: &str, data: &mut &[u8]) {
        let mut path_prefix = "./test_data/".to_owned();
        println!("writing: {:?}", data);
        path_prefix.push_str(topic);
        println!("path prefix: {}", path_prefix);
        if !Path::new(&path_prefix).is_dir() {
            fs::create_dir(path_prefix).expect("folder should be created");
        } else {
            println!("skipping dir creation");
        }
        // let mut new_path_prefix = path_prefix;
        let mut f = fs::File::create("./test_data/Dummy/000000.log").unwrap();
        let new_data = &[116, 101, 115, 116, 32, 97, 98, 50, 99];
        println!("new_data {:?}", print_type_of(new_data));
        f.write_all(b"new_data").expect("should have written to file");
        
    }
}

struct Producer<'a> {
    Topic: &'a str,
    writer: Writer,
}

impl Producer<'_> {
    fn produce(&self, data: String) {
        let raw_data = &mut data.as_bytes();
        println!("raw_data: {:?}", raw_data);
        self.writer.write(self.Topic, raw_data);
    }

    fn new(topic: &str, writer: Writer) -> Producer {
        return Producer {
            Topic: topic,
            writer: writer,
        };
    }
}

// stored in the .log file
#[derive(Copy, Clone, Debug)]
struct Record<'a> {
    base_offset: i32,
    create_time: i32,
    size: i32,
    position: i32,
    payload: Message<'a>,
}

struct Partition {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Topic {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let string = "Hello World";
    // print_type_of(string.as_bytes());

    let msg = Message {
        raw_data: string.as_bytes(),
    };

    let record = Record {
        base_offset: 0,
        create_time: 0,
        size: 2,
        position: 2,
        payload: msg,
    };

    // print_type_of(msg);
    // println!("{:?}", msg.raw_data);
    // println!("{:?}", str::from_utf8(msg.raw_data).unwrap());
    // println!("{:?}", record);

    // File::create("hello.txt");
    // let content = fs::read_to_string("hello.txt"); //.expect("Should have been able to read the file");
    // print_type_of(content);

    let w = Writer {};
    let p = Producer {
        Topic: "Dummy1",
        writer: w,
    };
    p.produce(record.to_string());

    /*
     * Init a producer
     * Set topic name
     * use the producer to write a message to log file
     */
}
