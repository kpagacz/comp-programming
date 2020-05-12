#include<stdio.h>
#include<string.h>

int main(void) {
    int tests;
    scanf("%i", &tests);
    char* days[] = {"Pn", "Wt", "Sr", "Cz", "Pt", "So", "Nd"};
    char day[3];
    int start_day_int, skip_days;

    for(int i = 0; i < tests; i++) {
        scanf("%s", &day);
        day[3] = '\0';

        for(int j = 0; j < 7; j++) {
            int comparison = strcmp(days[j], day);
            if(comparison == 0) {
                start_day_int = j;
                break;
            }
        }

        scanf("%i", &skip_days);

        printf("%s\n", days[(start_day_int + skip_days) % 7]);

    }


    return 0;
}