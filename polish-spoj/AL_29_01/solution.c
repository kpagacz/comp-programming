#include<stdio.h>

int main(void) {
    int players;
    int first_split_hours, first_split_minutes, first_split_seconds, 
        second_split_hours, second_split_minutes, second_split_seconds,
        third_split_hours, third_split_minutes, third_split_seconds,
        fourth_split_hours, fourth_split_minutes, fourth_split_seconds, 
        total_hours, total_minutes, total_seconds, 
        wanted_hours, wanted_minutes, wanted_seconds;
    int total_time_in_seconds, first_split_in_seconds, second_split_in_seconds, 
        third_split_in_seconds, fourth_split_in_seconds, wanted_in_seconds;
    
    int negative_split_players = 0, negative_split_players_succeded = 0, total_succeded = 0;

    scanf("%i", &players);

    for(int i = 0; i < players; i++) {
        scanf("%d:%d:%d %d:%d:%d %d:%d:%d %d:%d:%d %d:%d:%d %d:%d:%d",
            &first_split_hours, &first_split_minutes, &first_split_seconds,
            &second_split_hours, &second_split_minutes, &second_split_seconds,
            &third_split_hours, &third_split_minutes, &third_split_seconds,
            &fourth_split_hours, &fourth_split_minutes, &fourth_split_seconds,
            &total_hours, &total_minutes, &total_seconds,
            &wanted_hours, &wanted_minutes, &wanted_seconds);

        /*
        printf("%d:%d:%d %d:%d:%d %d:%d:%d %d:%d:%d %d:%d:%d %d:%d:%d\n", 
            first_split_hours, first_split_minutes, first_split_seconds,
            second_split_hours, second_split_minutes, second_split_seconds,
            third_split_hours, third_split_minutes, third_split_seconds,
            fourth_split_hours, fourth_split_minutes, fourth_split_seconds, 
            total_hours, total_minutes, total_seconds, 
            wanted_hours, wanted_minutes, wanted_seconds);
        */

        total_time_in_seconds = total_hours * 3600 + total_minutes * 60 + total_seconds;
        first_split_in_seconds = first_split_hours * 3600 + first_split_minutes * 60 + first_split_seconds;
        second_split_in_seconds = second_split_hours * 3600 + second_split_minutes * 60 + second_split_seconds;
        third_split_in_seconds = third_split_hours * 3600 + third_split_minutes * 60 + third_split_seconds;
        fourth_split_in_seconds = fourth_split_hours * 3600 + fourth_split_minutes * 60 + fourth_split_seconds;
        wanted_in_seconds = wanted_hours * 3600 + wanted_minutes * 60 + wanted_seconds;

        fourth_split_in_seconds = fourth_split_in_seconds - third_split_in_seconds;
        third_split_in_seconds = third_split_in_seconds - second_split_in_seconds;
        second_split_in_seconds = second_split_in_seconds - first_split_in_seconds;

        /*
        printf("%i %i %i\n", first_split_in_seconds, second_split_in_seconds, third_split_in_seconds, fourth_split_in_seconds);
        */
        if(total_time_in_seconds <= wanted_in_seconds) {
            total_succeded++;
        }

        if(first_split_in_seconds >= second_split_in_seconds && second_split_in_seconds >= third_split_in_seconds && 
            third_split_in_seconds >= fourth_split_in_seconds && first_split_in_seconds > fourth_split_in_seconds) {
                negative_split_players++;
                if(total_time_in_seconds <= wanted_in_seconds) {
                    negative_split_players_succeded++;
                }
        }
    }

    printf("%i/%i %i/%i\n", negative_split_players_succeded, negative_split_players, total_succeded - negative_split_players_succeded,
        players - negative_split_players);
    
    return 0;
}