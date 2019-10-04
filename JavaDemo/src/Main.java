import java.sql.Array;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.Stack;
import java.util.Vector;

public class Main {

    public static void main(String[] args) {

        System.out.println("begin demo...");

        {
            LinkedList linkList = new LinkedList();
            linkList.add(1);
            linkList.add(2);
            linkList.add(0, 4);
            linkList.addFirst(5);
            linkList.add(null);
            System.out.println(linkList);
        }

        {
            ArrayList arrayList = new ArrayList();
            arrayList.add(5);
        }

        {
            Vector vec = new Vector();
            vec.add(5);
            System.out.println(vec);
        }

        {
            Stack stack = new Stack();
            stack.push(5);
        }
    }
}
