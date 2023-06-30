mod kafka_core;

use std::fs;

struct Writer {}

impl Writer {
    fn write(topic: String, data: [u8]) {
        path_prefix = "./test_data/";
        fs::create_dir(path_prefix + topic);        
        f = fs::File("0000000.log");
        f.write(data)
    }
}