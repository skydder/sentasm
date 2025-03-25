int printf();

int main() {
    for (int i = 1; i < 40; i = i + 1) {
        int is_mul3 = (i % 3) == 0;
        int is_mul5 = (i % 5) == 0;
        int is_mul15 = is_mul3 & is_mul5;
        if (is_mul15) {
            printf("fizzbuzz\n");
        } else if (is_mul5) {
            printf("buzz\n");
        } else if (is_mul3) {
            printf("fizz\n");
        } else {
            printf("num\n");
        }
    }
}