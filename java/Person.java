import java.util.ArrayList;

class Person {
    String firstName;
    String lastName;
    List<String> locations;

    public Person(String firstName, String lastName) {
        this.firstName = firstName;
        this.lastName = lastName;
        this.locations = new ArrayList<>();
    }

    public void addLocation(String location) {
        this.locations.add(location);
    }

    public String fullName() {
        return String.format("%s %s", this.firstName, this.lastName);
    }

    public static void main(String[] args) {
        Person p = new Person("John", "Doe");
        p.addLocation("Somewhere");
        System.out.println(p.fullName());
    }
}
