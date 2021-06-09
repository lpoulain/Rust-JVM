import java.util.ArrayList;
import java.util.List;

class Person {
    String firstName;
    String lastName;
    List<String> locations;
    int age;

    public Person(String firstName, String lastName, int age) {
        this.firstName = firstName;
        this.lastName = lastName;
        this.age = age;
        this.locations = new ArrayList<>();
    }

    public void addLocation(String location) {
        this.locations.add(location);
    }

    public void printAgeInfo() {
        switch (this.age) {
            case 0:
            case 1:
            case 2:
            case 3:
            case 4:
            case 5:
            case 6:
            case 7:
            case 8:
            case 9:
            case 10:
            case 11:
            case 12:
            case 13:
            case 14:
            case 15:
            case 16:
            case 17:
                System.out.println("Minor");
                break;
            case 18:
                System.out.println("Adult");
                break;
            case 21:
                System.out.println("Can legally drink (in the U.S.)");
                break;
            case 40:
                System.out.println("40 is the new 30");
                break;
            case 65:
                System.out.println("Retirement age");
                break;
            case 100:
                System.out.println("Congratulations!");
                break;
            default:
                System.out.println(String.format("Aged %d", this.age));
        }
    }

    public void printLastNameInfo() {
        switch(this.lastName) {
            case "Smith":
                System.out.println("Will or Kevin?");
                break;
            case "Doe":
                System.out.println("Joe or Jane?");
                break;
            case "Gates":
                System.out.println("Bill??");
                break;
            default:
                System.out.println("Non-common name");
        }
    }

    public String fullName() {
        return String.format("%s %s", this.firstName, this.lastName);
    }

    public static void main(String[] args) {
        Person p = new Person("John", "Smith", 21);
        p.addLocation("Somewhere");

        System.out.println(p.fullName());
        p.printAgeInfo();
        p.printLastNameInfo();
    }
}
