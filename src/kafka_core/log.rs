

use std::path::Path;

struct Log {
    path_prefix: String,
}

impl Log {
    fn set_path_prefix(&self, path_prefix: String)  {
        &self.path_prefix = path_prefix;
    }
    
    fn validate_topic(topic: String) {
        topic_info = get_topic_info(topic)
    }
}




// Reads the folder structure to determine if topic was already created or not
// fn get_topic_info(topic: String) {
//     Path::    
// }