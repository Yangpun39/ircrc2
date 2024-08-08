#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_tokens() {
        let mut wallet = Wallet::new("owner".to_string());
        wallet.receive_tokens("sender".to_string(), 100);
        wallet.send_tokens("receiver".to_string(), 50);
        assert_eq!(wallet.get_balance(), 50);
    }
}
