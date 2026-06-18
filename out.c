#include <stdint.h>
#include <stdio.h>

int32_t r[256];
int32_t craw_fgo_target = -1;

int main(void) {
craw_line_0:
  craw_fgo_target = 8;
  goto craw_fgo_dispatch;
craw_line_1:
craw_line_2:
  r[2] = 65;
craw_line_3:
  putchar(r[2]);
craw_line_4:
craw_line_5:
craw_line_6:
craw_line_7:
craw_line_8:
  r[2] = 66;
craw_line_9:
  putchar(r[2]);
craw_line_10:
craw_line_11:
  r[2] = 67;
craw_line_12:
  putchar(r[2]);
  return 0;

craw_fgo_dispatch:
  switch (craw_fgo_target) {
    case 0: goto craw_line_0;
    case 1: goto craw_line_1;
    case 2: goto craw_line_2;
    case 3: goto craw_line_3;
    case 4: goto craw_line_4;
    case 5: goto craw_line_5;
    case 6: goto craw_line_6;
    case 7: goto craw_line_7;
    case 8: goto craw_line_8;
    case 9: goto craw_line_9;
    case 10: goto craw_line_10;
    case 11: goto craw_line_11;
    case 12: goto craw_line_12;
    default: return 1;
  }
}
