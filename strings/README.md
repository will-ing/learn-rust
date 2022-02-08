# Strings

**ASCII** - is a coding for integers to characters. American standard code

UTF-8 - Unicode transformation formal 8 bits. Over a million char and backwards compatible with ASCII.

&str - A view into a sequence of utf-8 encoded bytes of dynamic length. Can be stored on Binary, Stack or the Heap. Stores address and length of the string. Does not own the data.

String - Its growable, mutable representation of utf-8 encoded bytes on the stored only on the Heap. It stores the address, length and capacity.
