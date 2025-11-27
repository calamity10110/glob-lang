#include <math.h>
#include <string.h>

cs c:
    double calculate_distance(double x1, double y1, double x2, double y2) {
        double dx = x2 - x1;
        double dy = y2 - y1;
        return sqrt(dx*dx + dy*dy);
    }
    
    void reverse_string(char* str) {
        int len = strlen(str);
        for (int i = 0; i < len/2; i++) {
            char temp = str[i];
            str[i] = str[len-1-i];
            str[len-1-i] = temp;
        }
    }

mn main():
    distance = c.calculate_distance(0, 0, 3, 4)
    print("Distance:", distance)
    
    own text = "Hello, World!"
    c.reverse_string(ref text)
    print("Reversed:", text)
