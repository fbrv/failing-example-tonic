pub mod rpc {
    tonic::include_proto!("common");
    tonic::include_proto!("service");
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
