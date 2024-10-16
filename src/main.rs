use rpresence::{rpc::packet::Activity, RichClient};

fn main() {
    let mut client = RichClient::new(1219918645770059796);
    client.connect(true).unwrap();
    client.update(Activity::new().details("test")).unwrap();
}
