#include<iostream>
#include<stdlib.h>
#include<pthread.h>
#include<windows.h>
using namespace std;
#define MIN_PID 100
#define MAX_PID 1000
int threadVar =0;
pthread_mutex_t mutex;
