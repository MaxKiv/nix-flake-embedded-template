default:
    @just --list

run:
  cargo espflash flash --monitor

data:
  xdg-open ./data/esp32c3-schematic.pdf
  xdg-open ./data/esp32c3-datasheet_en.pdf
  xdg-open ./data/1.54inch_e-Paper_Datasheet.pdf
