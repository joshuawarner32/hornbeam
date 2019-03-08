#include "tests.h"
#include <wctype.h>

int main(int argc, char **argv)
{
     int error = 0, better = 0;
     utf8proc_int32_t c;

     (void) argc; /* unused */
     (void) argv; /* unused */

     /* some simple sanity tests of the character widths */
     for (c = 0; c <= 0x110000; ++c) {
          utf8proc_int32_t l = utf8proc_tolower(c);
          utf8proc_int32_t u = utf8proc_toupper(c);
          utf8proc_int32_t t = utf8proc_totitle(c);

          check(l == c || utf8proc_codepoint_valid(l), "invalid tolower");
          check(u == c || utf8proc_codepoint_valid(u), "invalid toupper");
          check(t == c || utf8proc_codepoint_valid(t), "invalid totitle");

          if (utf8proc_codepoint_valid(c) && (l == u) != (l == t)) {
               fprintf(stderr, "unexpected titlecase %x for lowercase %x / uppercase %x\n", t, l, c);
               ++error;
          }

          if (sizeof(wint_t) > 2 || c < (1<<16)) {
               wint_t l0 = towlower(c), u0 = towupper(c);

               /* OS unicode tables may be out of date.  But if they
                  do have a lower/uppercase mapping, hopefully it
                  is correct? */
               if (l0 != c && l0 != l) {
                    fprintf(stderr, "MISMATCH %x != towlower(%x) == %x\n",
                            l, c, l0);
                    ++error;
               }
               else if (l0 != l) { /* often true for out-of-date OS unicode */
                    ++better;
                    /* printf("%x != towlower(%x) == %x\n", l, c, l0); */
               }
               if (u0 != c && u0 != u) {
                    fprintf(stderr, "MISMATCH %x != towupper(%x) == %x\n",
                            u, c, u0);
                    ++error;
               }
               else if (u0 != u) { /* often true for out-of-date OS unicode */
                    ++better;
                    /* printf("%x != towupper(%x) == %x\n", u, c, u0); */
               }
          }
     }
     check(!error, "utf8proc case conversion FAILED %d tests.", error);

     /* issue #130 */
     check(utf8proc_toupper(0x00df) == 0x1e9e &&
           utf8proc_totitle(0x00df) == 0x1e9e &&
           utf8proc_tolower(0x00df) == 0x00df &&
           utf8proc_tolower(0x1e9e) == 0x00df &&
           utf8proc_toupper(0x1e9e) == 0x1e9e,
           "incorrect 0x00df/0x1e9e case conversions");
     utf8proc_uint8_t str_00df[] = {0xc3, 0x9f, 0x00};
     utf8proc_uint8_t str_1e9e[] = {0xe1, 0xba, 0x9e, 0x00};
     check(!strcmp((char*)utf8proc_NFKC_Casefold(str_00df), "ss") &&
           !strcmp((char*)utf8proc_NFKC_Casefold(str_1e9e), "ss"),
           "incorrect 0x00df/0x1e9e casefold normalization");

     printf("More up-to-date than OS unicode tables for %d tests.\n", better);
     printf("utf8proc case conversion tests SUCCEEDED.\n");
     return 0;
}
