## Pre-alpha

### 0.1.21
- Add MSWP for easy sprite management

### 0.1.20
- *BREAKING CHANGE*
- Atlas count changed from 2 to 4
- Atlas size changed from 8000 to 4000
- Bit mask and order changed for sprites, layer headers and contents
- Each sprite can use any atlas
- Add structs to make working with sprites and layers easier

### 0.1.19
- *BREAKING CHANGE*
- Increase stack to 1000b and change it's address
- Change total sizes to usize so they are the correct value of 65536 instead of 65535

### 0.1.18
- *BREAKING CHANGE*
- Fix issue with offset addressing (split reg into reg and ext reg)

### 0.1.17
- *BREAKING CHANGE* (MAJOR)
- Rename repo and dep to `platform`
- Remove useless op_byte_count method (offset addressing means that byte count is controlled by params now)
- Add datetime interrupt (it fires on date time changes greater than 1s, for example after resuming a mobile host or loading from a save state)

### 0.1.16
- Add PUSH and POP instructions at x70
- Add reg and num offset addressing

### 0.1.15
- Add count for controller types

### 0.1.14
- *BREAKING CHANGE*
- Add `DATETIME` and `RAND` mem addresses
- Allow 16 colours per palette by making 0,0,0 be transparent
- Most memory addresses increase 12 bytes
- Add bit masks for graphics

### 0.1.13
- *BREAKING CHANGE* (minor)
- Removed `SAVE_COUNT`, use `constants::SAVE_COUNT`
- Add method to check if mem address is expensive to update (for example if it controls a bank)
- Add `SAVE_CONTROL` 

### 0.1.12

- Add `EHALT` command
- Add `MAY_JMP_OPS` and `MUST_JMP_OPS` lists

### 0.1.11

- *BREAKING CHANGE*
- Add interrupt control
- Add controller sprite palette
- Add controller graphic sprite table + IDs
- Reduce background layer to 3 (from 4)
- Increase code and ram bank size to 8700b (from 8192)
- Increase stack to 900b (from 800)
- Increase reserved to 171b (from 37)
- Add interrupt control flags
- Add save count constant
- Increase sound to 30b (from 25)
- Add 4 keyboard controller types (for different layouts)

### 0.1.10

- Add SLEEP
- Add internal addresses (VLine)
- Add X,Y buttons
- Add controller type
- Add controller graphic addresses

### 0.1.9

- *BREAKING CHANGE*
- Fix mem TOTAL, by changing it to 0xFFFF 
- Add timer and interrupt addresses
- Reduce stack to 800 bytes

### 0.1.8

- *BREAKING CHANGE*
- Move register offset to VM
  - As language registers should be independent of VM implementation

### 0.1.7

- Correct typos in names

### 0.1.6

- Add register::names
- Make ppid fields public

### 0.1.5

- *BREAKING CHANGE*
- Change register method names to make purpose clearer
- Add registers::offset::from_id method
- Add struct for register apps, with to/from u8

### 0.1.4

- *BREAKING CHANGE*
- Replace panics with results

### 0.1.3

- *BREAKING CHANGE*
- Add more commands
  - ADDC
  - SUBC
- Remove half carry flag, left as reserved for now
- Mark 13 bytes from stack as reserved, stack is 850 bytes

### 0.1.2

- Add missing commands   
  - JBS
  - JBC
- Correct OP names

### 0.1.1

- *BREAKING CHANGE*
- Move code from unnecessary `mod`

### 0.1.0

- Initial release
- Moved from `maikor-vm` as ops, etc will be needed by projects that don't need VM structs, etc