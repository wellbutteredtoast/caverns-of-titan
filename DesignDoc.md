# Caverns of Titan - DD
<!-- This makes heavy use of the GitHub Markdown secret sauce -->
![Static Badge](https://img.shields.io/badge/built_with-rust-orange?logo=rust)
![Static Badge](https://img.shields.io/badge/built_for-MVM28-red?link=https%3A%2F%2Fitch.io%2Fjam%2Fmetroidvania-month-28)
![Static Badge](https://img.shields.io/badge/license-MIT-blue)

## Preamble

> [!IMPORTANT]
> Everything in this document is subject to change with little to no notice!

This document is not a traditional design document, more like an organized mess of what I'm planning to add, both during the game jam and post-jam. (if all goes well) Mostly covering core mechanics, designs of lifeforms to find and areas to explore, the usual things.

# Core Mechanics

## Movement

Movement follows your typical left-right movement with sprinting, jumping, and crouching for a very slightly better movement experience. There *is* gravity, of course, but considerably weaker than Earth gravity. On Earth, gravity is **about** 9.81m/s<sup>2</sup>, however on Titan the gravity is **roughly** 1.53m/s<sup>2</sup>. But for the sake of not making everything super floaty and negating a large amount of the challenge, I will be ignoring the actual gravity values and making it about 90% of Earth gravity, or about 8.82m/s<sup>2</sup>, providing a slightly floaty feeling without invalidating a large amount of the game. How am I going to implement *proper* physics? I have no idea, but the answer is with duct tape and hope.

## Science Collection

This is a novel mechanic, when interacting with some form of plant-life or animal life, you can collect a sample of it and transmit the data to recieve experience to level up. While scientific data collection is not necessarily a *key aspect* of the game, it is one of the only ways to actually gain large amounts of XP without endless and agonizing grinding. Scientific data (Sd) is 1:1 with experience (XP). There is also a one-time **first-time find bonus** in which you gain an added 50% Sd the first time you find something new, this applies to all lifeforms you can harvest data from, and only happens once, all other times will result in the same base amount of science

> [!NOTE]
> This may not be fully implemented in the **game jam** version of Caverns of Titan due to possible time constraints.

## Upgrades

> [!NOTE]
> No design notes as of **05/12/2025**

## Combat

> [!NOTE]
> No design notes as of **05/12/2025**