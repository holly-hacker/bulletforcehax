import 'packet_reader_test.dart' as packet_reader_tests;
import 'protocol_reader_test.dart' as protocol_read_tests;
import 'protocol_writer_test.dart' as protocol_write_tests;

void main() {
  protocol_read_tests.main();
  protocol_write_tests.main();
  packet_reader_tests.main();
}
