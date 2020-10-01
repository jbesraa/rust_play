Object Vehicle {
	Types: Private/TwoWheeled/Truck,

}

- Only one Vehicle can get washed at a time
- Only one Vehicle can get gas at a time
- A Vehicle can ask for a Wash or and Gas
- Each Vehicle has number, type, service type(gas/wash)


Vehicle      washtime gastime
TwoWheeled    2        1
private       4        3
truck         6        5

write a program to monitor the actions performd in the station
Example: Vehicle xxx starts cleaning/refueling in time yyy.

roles:
- no time waste for workers and no unneccesary waiting time for cars
- if two Vehicles need to enter the same line and one of them finished previous 
action, they are priortised 
- Vehicles arrive in time 0

Input: 
[
	{type: 'motorcycle' , action: 'RC'},
	{type: 'trailer' , action: 'R'},
	{type: 'motorcycle' , action: 'RC'},
	{type: 'private' , action: 'C'},
	{type: 'private' , action: 'C'},
	{type: 'private' , action: 'R'},
	{type: 'private' , action: 'R'},
	{type: 'motorcycle' , action: 'R'},
	{type: 'motorcycle' , action: 'R'},
	{type: 'private' , action: 'RC'},
	{type: 'trailer' , action: 'C'},
	{type: 'motorcycle' , action: 'R'},
	{type: 'private' , action: 'R'},
	{type: 'trailer' , action: 'RC'},
	{type: 'private' , action: 'R'},
]