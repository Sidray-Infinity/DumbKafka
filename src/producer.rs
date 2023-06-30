/*
Whenever a message is recevied
- Get the active segment
- Get the latest offset
- Decide if new segment is needed ? Create new segment : continue
- Write to log file
- Decide if index file needs to be updated ? Create entry in index & timeindex file : continue
*/

struct Producer {
    Topic: String,
    writer: Writer,
}

impl Producer {
    fn produce(&self, data: String) {
        raw_data = data.as_bytes();
        self.writer.write(topic, raw_data);
    }

    fn new(topic: String, writer: Writer) -> Producer {
        return Producer {
            Topic: string,
            writer: Writer,
        };
    }
}
