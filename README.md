# [WIP] Weekend DB - Fast distributed kv store

Trying to create a production ready in-memory store which will work as a faster alternative to Redis or memcache. It follows the shared nothing architecture described in [bloom lang](http://bloom-lang.net/calm/) and [Anna Paper](https://dsf.berkeley.edu/jmh/papers/anna_ieee18.pdf). 

##Todo

- [x] Lattice data structures
- [x] Consistent Hash Ring
- [ ] Seed Node
- [ ] Node join/leave
- [ ] Put/Get Request
...

##Research

- http://db.cs.berkeley.edu/jmh/papers/anna_ieee18.pdf [must read]
- http://bloom-lang.net/calm/ [must read]
- http://www.vikrams.io/papers/anna-vldb19.pdf [must read]
- http://www.jmfaleiro.com/pubs/latch-free-cidr2017.pdf [must read]
- https://arxiv.org/pdf/1901.01930.pdf
- https://www.cs.cornell.edu/projects/ladis2009/papers/lakshman-ladis2009.pdf
- https://www.scylladb.com/
- https://github.com/papers-we-love/papers-we-love/blob/master/datastores/dynamo-amazons-highly-available-key-value-store.pdf
