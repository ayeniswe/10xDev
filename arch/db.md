# Database Paradigms

## In Memory

### Why we care

- Has the ability to be fast
in context of *read/write* because
data is stored in **Random Access Memory (RAM)** instead of **Disk**
- Has similarties to objects in javascript

    ```javascript
    { id: 10, location: New York }
    ```

- Can be and usually used in conjuction with other databases

### Tradeoffs

- No **Queries** or **Joins**
- Only key-values pairs are supported
- Not ACID compliance

### Use Case

- Caching
- Publisher and Subscription [(Pub/Sub)](design-patterns.md#pubsub)
- Leaderboards

### Example DB(s)

- Redis

## Wide Column

### Why we care

- **Scalable** because of no schema
- Has the ability to **query**
- Has rows and columns
- Data is [unstructured](db.md#structured-vs-unstructured)

### Tradeoffs

- No **Joins**
- Not ACID compliance

### Use Case

- High write, low read
- Histrorical records
- Time-series

### Example DB(s)

- Cassandra
- Apache HBASE

## Document

### Why we care

- Similiar to [Wide Column](#wide-column)

### Tradeoffs

- No **Joins**
- Not truly relational
- Fails in efficiency when having High Writes
- Not ACID compliance

### Use Case

- Apps
- Games
- IOT

### Example DB(s)

- MongoDB
- Firestore
- RoachDB

## Relational

### Why we care

- ACID compliance
  - Atomicity
  - Consistency
  - Isolation
  - Durability
- Has a schema
  - having data shaped in a predefined way

### Tradeoffs

- Scaling problems, but modern DBs may solve
- Does not work well with unstructured data

### Use Case

- Social Media Apps
- Finacials Institutions

### Example DB(s)

- PostgreSQL
- MySQL
- CockroachDB (Modern)

## Graph

### Why we care

- Nodes seen as **data** and edges as **relationships**
- Simplifies many-to-many relationships
  - You will notice this is needed when many queries are using **Join** majority
  of the time

### Tradeoffs

- Scaling problems, but modern DBs may solve
- Does not work well with unstructured data

### Use Case

- Social Media Apps
- Finacials Institutions
- Graphs
- Knowledge Graphs
- Recommedation Engines

### Example DB(s)

- Neo4j
- DGraph

## Vector

### Why we care

- High dimensional and complex data analysis
  - images, videos, and/or text embeddings
- Ease of intergration; especially ML
- Similarity indexing

### Tradeoffs

- Possible cost due to growing data size
- Dimension increases leads to degradation
  - Mitgation solutions are implemented

### Use Case

- Search indexing
- Fraud detection
- ML

### Example DB(s)

- ChromaDb
- Pinecone
  