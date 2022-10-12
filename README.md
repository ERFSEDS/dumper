
# Dumper protocol

To support normal logging, the application will print normal ACSII logs. These should be printed by dumper
The start of a binary data dump will begin with `DUMPER BEGIN:\r\n` text followed by the dumper header block
The header block contains 3, 32bit little-endian words storing the start page index, end page index, and page size
All other 32 bit integers in this format are encoded in little endian unless specified otherwise


Next zero or more page entries are encoded:

Each page entry starts with a 32 bit number designating the type of page entry followed by some entry specific data.
ENTRY_DATA = number: 0xDEADBEEF - the following page size bytes are the next page in the dump

LOG_DATA = 0xF0F0F0F0 - the following 32 bits store the length of the log message (n).
The following n bytes are a ASCII log message that should be printed to the user

