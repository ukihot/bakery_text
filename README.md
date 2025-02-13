# Bakery Text

Bake bread before opening the bakery. This is a CUI-based bakery game called "TEXT". Use commands to bake bread based on incoming information.

## Gameplay

Enemies may cause trouble during preparation. Handle them as they come. The game is like a tower defense game. Manage multiple ovens (terminals) and aim for a high score.

## Game Elements

- Terminals have modes: Commander and Observer. The terminal in focus is Commander mode, others are Observer mode.
- In Observer mode, information flows continuously. Switching to Commander mode allows command input.
- Ingredients and artifacts are not duplicated across terminals. Use the `mv` command to move them to another terminal.

## General Commands

- `ls`: Observe the current state of the section.
- `mv`: Move materials or artifacts to another terminal.
- `shoo`: Remove uninvited guests from the section.

## Component Responsibility

Each component has a single responsibility. Each section is dedicated to a specific task and collaborates with other sections to produce bread.

## Production Sections and Commands

| ID  | Section         | Abbreviation | Purpose                                   | Commands             | Receives from |
| --- | --------------- | ------------ | ----------------------------------------- | -------------------- | ------------- |
| 00  | Purchasing      | PS           | Order and manage procurement of materials | `order`              | (None)        |
| 01  | Pantry          | PN           | Store raw materials and ingredients       |                      | PS            |
| 02  | Mixing          | MX           | Mix ingredients to form dough             | `mx`                 | ST            |
| 03  | Cooling         | CL           | Let dough rise / Cool baked bread         | `pf`, `cl`           | MX, SH, BK    |
| 04  | Shaping         | SH           | Shape the dough                           | `div`, `roll`        | CL            |
| 05  | Baking          | BK           | Bake the dough                            | `bk`                 | SH            |
| 06  | Packaging       | PK           | Package bread for sale                    | `pack`, `label`      | CL            |
| 07  | Quality Control | QC           | Check bread for quality                   | `inspect`, `report`  | PK            |
| 08  | Stockroom       | ST           | Store packaged bread                      | `store`, `inv`       | QC            |
| 09  | Sales Front     | SF           | Sell bread                                | (Observer mode only) | ST            |
| 10  | Waste Station   | WS           | Handle waste                              | `dispose`            | (Any)         |
| 11  | Utility         | UT           | Manage water, electricity, and gas supply | (Observer mode only) | (None)        |

### Section Descriptions

- **Mixing (MX)**: Mix ingredients to form dough. This is the first step in bread making.
- **Cooling (CL)**: Let the dough rise (first fermentation) or cool baked bread. This step is crucial for the dough to develop properly.
- **Shaping (SH)**: Shape the dough after the first fermentation. This step prepares the dough for baking.
- **Baking (BK)**: Bake the shaped dough. This step transforms the dough into bread.
- **Sales Front (SF)**: Display the current score. This section operates in Observer mode only.
- **Packaging (PK)**: Package the baked bread for sale. This step includes labeling and preparing the bread for the stockroom.
- **Stockroom (ST)**: Store packaged bread. This step involves inventory management to ensure fresh bread is available.
- **Quality Control (QC)**: Check the bread for quality. This step ensures that only high-quality bread reaches the customers.
- **Waste Station (WS)**: Handle waste. This step involves disposing of any waste generated during the production process.

### Section Errors

- **Mixing (MX)**: None (foreign object check not possible here)
- **Cooling (CL)**: None (potential for improper cooling)
- **Shaping (SH)**: None (potential for improper shaping)
- **Baking (BK)**: Fire hazard (overheating)
- **Sales Front (SF)**: None (issues warnings only)
- **Packaging (PK)**: None (potential for improper packaging)
- **Stockroom (ST)**: None (potential for inventory mismanagement)
- **Quality Control (QC)**: Various errors including foreign objects, fermentation issues, baking issues (overbaked or underbaked), shaping issues, and packaging issues. All aspects are inspected.

### Uninvited Guests

Random intruders may enter any section at random times and cause trouble. They can:

- Attempt to break things.
  - If left unattended for too long, they will cause damage requiring repairs.
- Place dummies, which can be recognized by the `ls` command.
  - fake ingredients not procured by purchasing, will not cause errors in processes but will be flagged as defects in QC.
- Eat ingredients or products.

To remove an intruder, focus on the affected section and use the `shoo` command in Commander mode.
