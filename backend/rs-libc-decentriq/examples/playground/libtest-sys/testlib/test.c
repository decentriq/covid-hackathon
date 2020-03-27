#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pthread.h> 

#include "test.h"

void test_function(int test) {
    rust_test_function(test);
}

void test_file(void) {
    FILE *fp;
    fp = fopen("/tmp/test.txt", "w+");
    fprintf(fp, "This is testing for fprintf...\n");
    fputs("This is testing for fputs...\n", fp);
    fclose(fp);
}

int accum = 0;
pthread_mutex_t lock; 

void* thread_func(void *arg) 
{
    for (int i=0; i<10; i++) {
	pthread_mutex_lock(&lock); 
	accum--;
	pthread_mutex_unlock(&lock); 
    }
    puts("Printing from Thread"); 
} 
   
void test_pthread(void) {
    pthread_t thread_id; 
    pthread_mutex_init(&lock, NULL);
    puts("Before Thread accum"); 
    test_function(accum);
    pthread_create(&thread_id, NULL, &thread_func, NULL); 
    for (int i=0; i<10; i++) {
	    pthread_mutex_lock(&lock); 
	    accum++;
	    pthread_mutex_unlock(&lock); 
    }
    pthread_join(thread_id, NULL); 
    puts("After Thread accum"); 
    test_function(accum);
    pthread_mutex_destroy(&lock); 
}
