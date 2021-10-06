#ifndef PLAYER_H
#define PLAYER_H
#include "raylib.h"

typedef struct {
    Rectangle rect;
    bool blocking;
    Color color;
} EnvItem;

typedef struct {
    Vector2 pos;
    Vector2 vel;
    float speed;
    bool canJump;
} Player;


void UpdatePlayer(Player *player, EnvItem *envItems, int envItemsLength, float delta);
void UpdatePlayerCamera(Camera2D *camera, Player *player, float delta, int width, int height);

#endif
