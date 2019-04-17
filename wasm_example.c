#include <stdio.h>
#include <stdlib.h>
#include <emscripten/emscripten.h>

int** ADJACENTS;
int ADJACENTS_LENGTH;

// Checks if the asigned node has the same color as it's neighbours
int works(int node, int* colored) {
  int other;
  for (size_t i = 0; i < ADJACENTS_LENGTH; i++) {
    if (ADJACENTS[i][0] == node) {
      int other = ADJACENTS[i][1];
    }
    if (ADJACENTS[i][1] == node) {
      int other = ADJACENTS[i][0];
    }
    if ((ADJACENTS[i][0] == node || ADJACENTS[i][1] == node) && colored[other] == colored[node]){
      return 0;
    }
  }
  return 1;
}

// Backtrack loop that assigns colors to all nodes
int backtrack(int colors, int *left_length, int *colored) {
  int next = *left_length;
  *left_length--;
  for (size_t i = 1; i <= colors; i++) {
    colored[next] = i;
    *left_length--;
    if (works(next, colored)) {
      if (backtrack(colors, left_length, colored)) {
        return 1;
      }
    }

    colored[next] = 0;
  }
  *left_length++;
  return 0;
}

// Call from JS to check if the graph defined by adjacency_list is k coloreable to a given k
// adjacency list must be an array of (i, j) pairs that denote that i is adjacent to j.
int EMSCRIPTEN_KEEPALIVE is_k_coloreable(int k, int nodes, int** adjacency_list, int adjacency_length){
  ADJACENTS = adjacency_list;
  ADJACENTS_LENGTH = adjacency_length;
  int colored[nodes];
  int left_length = nodes-1;
  return backtrack(k, &left_length, colored);
}

int factorial(int n) {
  if (n == 0)
    return 1;
  else
    return n * factorial(n-1);
}

 
int main(int argc, char ** argv) {
  int number = 8;
  int fact = factorial(number);
  printf("The factorial of %d is %d   \n", number, fact);

  // Initializing a matrix
  int adj_length = 10;
  int **adj = malloc(sizeof(int*)*10);
  for (size_t i = 0; i < 10; i++) {
    adj[i] = malloc(sizeof(int) * 2);
  }
  int nodes = 4;

  adj[0][0] = 0;
  adj[0][1] = 1;

  adj[1][0] = 0;
  adj[1][1] = 2;

  adj[2][0] = 0;
  adj[2][1] = 3;
  
  adj[3][0] = 1;
  adj[3][1] = 0;

  adj[4][0] = 1;
  adj[4][1] = 2;

  adj[5][0] = 2;
  adj[5][1] = 0;

  adj[6][0] = 2;
  adj[6][1] = 1;

  adj[7][0] = 2;
  adj[7][1] = 3;
  
  adj[8][0] = 3;
  adj[8][1] = 0;

  adj[9][0] = 3;
  adj[9][1] = 2;

  printf("Is the graph 3-coloreable? %d", is_k_coloreable(3, nodes, adj, adj_length));

}
