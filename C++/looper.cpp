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
	//int num;
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
	bool loop_found = false;
	int loops_tot = 0;
	do {
		for (ROPE &rope: ropes)
		{
			if (rope.marker < 1)
			{
				loop_found = true;
				loops_tot++;
				rope.marker = loops_tot;
				// move left or right, doesn't matter
				ROPE &next_rope = ropes[rope.right_con.rope_num];
				next_rope.marker = loops_tot;
				while (1)
				{
					if (next_rope.marker == loops_tot)
					{
						// a loop completed
						break;
					}
					else
					{
					 			// which side usage for calculating next
						next_rope.marker = loops_tot;
						switch (rope.right_con.side)
						{
							case Side::left:
								next_rope = ropes[next_rope.left_con.rope_num];
								cout << "Next rope " << next_rope.left_con.rope_num << " from left\n";
								break;
							case Side::right:
								next_rope = ropes[next_rope.right_con.rope_num];
								cout << "Next rope " << next_rope.right_con.rope_num << " from right\n";
								break;
						}
					}
				}
			}
			else
				loop_found = false;
		}
	} while (loop_found);
	cout << "Found loops " << loops_tot << "\n";
	cout << "All done\n";
}