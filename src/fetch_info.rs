pub trait FetchSystemInfos {
    fn get_os(self) -> String;
    fn get_user(self) -> String;
    fn get_cpu(self) -> String;
}
