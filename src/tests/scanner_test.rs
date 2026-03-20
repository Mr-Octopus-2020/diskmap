use crate::scanner::{ScanEvent, Scanner};
use std::path::PathBuf;

const TEST_DIR: &str = "src";

#[test]
fn test_scanner_events() {
        // We run this test in the src directory.
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(TEST_DIR);

        let scanner = Scanner::new(&path);
        let events: Vec<ScanEvent> = scanner.collect();

        // Verify first event is EnterDir with "src"
        assert!(matches!(events.first(), Some(ScanEvent::EnterDir(name)) if name == TEST_DIR));

        // Verify last event is ExitDir
        assert_eq!(events.last(), Some(&ScanEvent::ExitDir));

        // Verify presence of main.rs
        let has_main = events
                .iter()
                .any(|e| matches!(e, ScanEvent::File(name, _) if name == "main.rs"));
        assert!(has_main);
}
