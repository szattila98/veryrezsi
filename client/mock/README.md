# What is this

Mock database entities to give a foundation for frontend development, without creating any backend.
These should evolve into backend response like structures, as backend responses usually look different than the row data in the databse. For example:
	- Ids should not be passed to frontend (if it is possible)
	- Multiple entities can be aggregated into one response, insted of using foreign keys.
      	-  A user and its expenses can be in one object.


- In the _entities_ folder we store the data just like in the database. (Used for brainstorming and prototyping)
- In the _api_ folder we store the data just like how we want to get from the backend. 
These can be used to test frontend code and demo its functionality. (Used for Development)


# How it was created

- These are based on the _initial_ version of ER diagram
- Used [Mockaroo|https://www.mockaroo.com/] to generate repetitive data in creative manner.