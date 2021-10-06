#include "player.h"
#include "raylib.h"

#define PLAYER_SPEED 500.0f
#define G 1000.0f

void UpdatePlayerCamera(Camera2D *camera, Player *player, float delta, int width, int height) {
    camera->offset = (Vector2){width / 2.0f, height / 2.0f};
    camera->target = player->pos;
}

void UpdatePlayer(Player *player, EnvItem *envItems, int envItemsLength, float delta) {
    if (IsKeyDown(KEY_A)) player->pos.x -= PLAYER_SPEED*delta;
    if (IsKeyDown(KEY_D)) player->pos.x += PLAYER_SPEED*delta;
    if (IsKeyDown(KEY_SPACE) && player->canJump) {
        player->speed = -PLAYER_SPEED;
        player->canJump = false;
    }

    if (IsKeyDown(KEY_W) && player->canJump) {
        player->speed = -PLAYER_SPEED;
        player->canJump = false;
    }

    int hitObstacle = 0;
    for (int i = 0; i < envItemsLength; i++) {
        EnvItem *ei = envItems + i;
        Vector2 *p = &(player->pos);
        if (ei->blocking && ei->rect.x <= p->x && ei->rect.x + ei->rect.width >= p->x && ei->rect.y >= p->y && ei->rect.y < p->y + player->speed*delta) {
            hitObstacle = 1;
            player->speed = 0.0f;
            p->y = ei->rect.y;
        }
    }

    if (!hitObstacle) {
        player->pos.y += player->speed*delta;
        player->speed += G*delta;
        player->canJump = false;
    } else {
        player->canJump = true;
    }
}
