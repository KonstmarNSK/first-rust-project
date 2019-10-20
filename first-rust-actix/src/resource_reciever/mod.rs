pub trait ResourceReciever{
    fn recieve(addr: &str) -> String;
}

pub fn getResourceReciever() -> ResourceReciever {

}