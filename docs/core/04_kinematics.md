# Аналитическая кинематика

Прямой расчёт вращений без итерационного IK. Переводит позиции суставов в 56 локальных кватернионов стандарта `SkeletonProfileHumanoid`.

## 1. Построение базиса

Для каждой кости строится глобальная матрица вращения:

1. Нормированный вектор от начала кости к дочернему суставу.
2. Вспомогательная ось плоскости:
   - для таза поперечный вектор между бёдрами или плечами задаёт ось `Right`, а векторное произведение с осью позвоночника даёт `Forward`.
   - для конечностей нормаль плоскости сгиба сустава, например, плоскость плечо-локоть-запястье.
   - если конечность прямая или векторное произведение 0, то нужно брать вспомогательную ось родителя для избежания `NaN`.
3. Если дочерний сустав в `UnifiedBodyPose` равен `None`, то глобальное вращение кости приравнивается к глобальному вращению родителя, что даёт локальное вращение `Quat::IDENTITY`.

## 2. Перевод из глобального в локальное пространство родителя

В Godot вращение каждой кости задаётся локально относительно родительской кости и её `Rest Pose`. Соответственно, каждую дочернюю кость нужно переводить в локальное пространство по формуле:

`Q_local = (Q_parent_global * Q_rest).inverse() * Q_child_global`

## 3. Таблица индексов стандарта `SkeletonProfileHumanoid`

| Индекс | Кость                   | Родитель                     |
| ------ | ----------------------- | ---------------------------- |
| 0      | Root                    | -                            |
| 1      | Hips                    | 0 (Root)                     |
| 2      | Spine                   | 1 (Hips)                     |
| 3      | Chest                   | 2 (Spine)                    |
| 4      | UpperChest              | 3 (Chest)                    |
| 5      | Neck                    | 4 (UpperChest)               |
| 6      | Head                    | 5 (Neck)                     |
| 7      | LeftEye                 | 6 (Head)                     |
| 8      | RightEye                | 6 (Head)                     |
| 9      | Jaw                     | 6 (Head)                     |
| 10     | LeftShoulder            | 4 (UpperChest)               |
| 11     | LeftUpperArm            | 10 (LeftShoulder)            |
| 12     | LeftLowerArm            | 11 (LeftUpperArm)            |
| 13     | LeftHand                | 12 (LeftLowerArm)            |
| 14     | LeftThumbMetacarpal     | 13 (LeftHand)                |
| 15     | LeftThumbProximal       | 14 (LeftThumbMetacarpal)     |
| 16     | LeftThumbDistal         | 15 (LeftThumbProximal)       |
| 17     | LeftIndexProximal       | 13 (LeftHand)                |
| 18     | LeftIndexIntermediate   | 17 (LeftIndexProximal)       |
| 19     | LeftIndexDistal         | 18 (LeftIndexIntermediate)   |
| 20     | LeftMiddleProximal      | 13 (LeftHand)                |
| 21     | LeftMiddleIntermediate  | 20 (LeftMiddleProximal)      |
| 22     | LeftMiddleDistal        | 21 (LeftMiddleIntermediate)  |
| 23     | LeftRingProximal        | 13 (LeftHand)                |
| 24     | LeftRingIntermediate    | 23 (LeftRingProximal)        |
| 25     | LeftRingDistal          | 24 (LeftRingIntermediate)    |
| 26     | LeftLittleProximal      | 13 (LeftHand)                |
| 27     | LeftLittleIntermediate  | 26 (LeftLittleProximal)      |
| 28     | LeftLittleDistal        | 27 (LeftLittleIntermediate)  |
| 29     | RightShoulder           | 4 (UpperChest)               |
| 30     | RightUpperArm           | 29 (RightShoulder)           |
| 31     | RightLowerArm           | 30 (RightUpperArm)           |
| 32     | RightHand               | 31 (RightLowerArm)           |
| 33     | RightThumbMetacarpal    | 32 (RightHand)               |
| 34     | RightThumbProximal      | 33 (RightThumbMetacarpal)    |
| 35     | RightThumbDistal        | 34 (RightThumbProximal)      |
| 36     | RightIndexProximal      | 32 (RightHand)               |
| 37     | RightIndexIntermediate  | 36 (RightIndexProximal)      |
| 38     | RightIndexDistal        | 37 (RightIndexIntermediate)  |
| 39     | RightMiddleProximal     | 32 (RightHand)               |
| 40     | RightMiddleIntermediate | 39 (RightMiddleProximal)     |
| 41     | RightMiddleDistal       | 40 (RightMiddleIntermediate) |
| 42     | RightRingProximal       | 32 (RightHand)               |
| 43     | RightRingIntermediate   | 42 (RightRingProximal)       |
| 44     | RightRingDistal         | 43 (RightRingIntermediate)   |
| 45     | RightLittleProximal     | 32 (RightHand)               |
| 46     | RightLittleIntermediate | 45 (RightLittleProximal)     |
| 47     | RightLittleDistal       | 46 (RightLittleIntermediate) |
| 48     | LeftUpperLeg            | 1 (Hips)                     |
| 49     | LeftLowerLeg            | 48 (LeftUpperLeg)            |
| 50     | LeftFoot                | 49 (LeftLowerLeg)            |
| 51     | LeftToes                | 50 (LeftFoot)                |
| 52     | RightUpperLeg           | 1 (Hips)                     |
| 53     | RightLowerLeg           | 52 (RightUpperLeg)           |
| 54     | RightFoot               | 53 (RightLowerLeg)           |
| 55     | RightToes               | 54 (RightFoot)               |
