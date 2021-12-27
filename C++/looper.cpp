#include <iostream>
#include <fstream>
using namespace std;

enum Side
{
	left, right, not_connected
};

struct TIE
{
	int rope_num;
	Side side;
};

struct ROPE
{
	TIE left_con;
	TIE right_con;
	int marker;
};

ROPE ropes[50];
TIE selected;
int choices = 100;

int main(int argc, char *argv[])
{
	if (argc == 0)
		cout << "No args, assumed 50\n";
	else
		cout << argv[0] << " ropes in the bag\n";
	cout << "ropes " << sizeof ropes << "\n";
	srand(time(NULL));

	for (int c = 0; c < sizeof(ropes) / sizeof(ROPE); c++)
	{
		ropes[c].left_con.rope_num = -1;	// = new ROPE();
		ropes[c].right_con.rope_num = -1;
		ropes[c].marker = -1;
	}

	choices--;
	while (choices)
	{
		int randend = rand() % choices + 1;
		cout << "From " << choices << " choices\n";
		for (ROPE &rope: ropes)
		{
			int index = &rope - &ropes[0];
			if (rope.left_con.rope_num == -1)
			{
			 	// no one connected
				randend--;
				if (randend == 0)
				{
				 		// this one
					if (selected.rope_num > -1)
					{
					 			// one side already selected to be connected 
						rope.left_con.rope_num = selected.rope_num;
						rope.left_con.side = selected.side;
						switch (selected.side)
						{
							case Side::left:
								ropes[selected.rope_num].left_con.rope_num = index;
								ropes[selected.rope_num].left_con.side = Side::left;
								break;
							case Side::right:
								ropes[selected.rope_num].right_con.rope_num = index;
								ropes[selected.rope_num].right_con.side = Side::left;
								break;
						}

						selected.rope_num = -1;
						selected.side = Side::not_connected;
					}
					else
					{
						selected.rope_num = index;
						selected.side = Side::left;
						rope.left_con.rope_num = index;
					}
					choices--;
					break;	// loop
				}
			}
			if (rope.right_con.rope_num == -1)
			{
			 	// no one connected
				randend--;
				if (randend == 0)
				{
				 		// this one
					if (selected.rope_num > -1)
					{
					 			// connected
						rope.right_con.rope_num = selected.rope_num;
						rope.right_con.side = selected.side;
						switch (selected.side)
						{
							case Side::left:
								ropes[selected.rope_num].left_con.rope_num = index;
								ropes[selected.rope_num].left_con.side = Side::right;
								break;
							case Side::right:
								ropes[selected.rope_num].right_con.rope_num = index;
								ropes[selected.rope_num].right_con.side = Side::right;
								break;
						}

						selected.rope_num = -1;
						selected.side = Side::not_connected;
					}
					else
					{
						selected.rope_num = index;
						selected.side = Side::right;
						rope.right_con.rope_num = index;
					}
					choices--;
					break;
				}
			}
		}
	}
	cout << "Finding loops\n";
	for (ROPE &rope: ropes)
	{
		cout << "Rope " << &rope - &ropes[0] << " left " << rope.left_con.rope_num << " right " << rope.right_con.rope_num << "\n";
	}
	// find all loops 
	int loops_tot = 0;
	int sing_loops = 0;
	for (ROPE &rope: ropes)
	{
		if (rope.marker < 1)
		{
			loops_tot++;
			rope.marker = loops_tot;
			// move left or right, doesn't matter
			ROPE& next_rope = ropes[rope.right_con.rope_num];
			Side& next_side = rope.right_con.side;
			if (next_rope.marker > 0)
  				sing_loops++;
			while (next_rope.marker < 1)
			{
			 	// which side usage for calculating next
				next_rope.marker = loops_tot;
				switch (next_side)
				{
					case Side::right:
						next_rope = ropes[next_rope.right_con.rope_num];
						next_side = next_rope.right_con.side;
						cout << "Next rope " << next_rope.right_con.rope_num << " from right in loop " << next_rope.marker <<"\n";
						break;
					case Side::left:
						next_rope = ropes[next_rope.left_con.rope_num];
						next_side = next_rope.left_con.side;
						cout << "Next rope " << next_rope.left_con.rope_num << " from left in loop " << next_rope.marker <<"\n";
						break;
				}
			}
		}
	}

	cout << "Found loops " << loops_tot << " and single " << sing_loops << "\n";
	cout << "All done\n";
}
