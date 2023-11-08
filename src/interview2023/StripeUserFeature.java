package interview2023;

import java.util.*;

public class StripeUserFeature {


    private List<String> get_user_features(User user, List<Feature> features) {
        List<String> ret = new ArrayList<>();

        for(Feature feature : features) {
            if (!feature.isAbTest()) continue;
            if (feature.getLocations() != null && feature.getLocations().contains(user.getLocation())) {
                ret.add(feature.getID());
            }
        }
        return ret;
    }

    public static void main(String[] args) {
        User user = new User(1, "Abby", "US");
        Feature f1 = new Feature("us_annual_sale", new HashSet<>(Arrays.asList("US")), true);
        Feature f2 = new Feature("ca_annual_sale", new HashSet<>(Arrays.asList("CA")), true);
        Feature f3 = new Feature("us_annual_sale_no_ab", new HashSet<>(Arrays.asList("US")), false);

        StripeUserFeature stripeUserFeature = new StripeUserFeature();
        List<String> userFeatures = stripeUserFeature.get_user_features(user, Arrays.asList(f1, f2, f3));

        for(String s : userFeatures) System.out.println("use have feature " + s);
    }
}

class User {
    private int ID;
    private String name;
    private String location;

    public User(int ID, String name, String location) {
        this.ID = ID;
        this.name = name;
        this.location = location;
    }

    public int getID() {
        return ID;
    }

    public String getName() {
        return name;
    }

    public String getLocation() {
        return location;
    }
}

class Feature {
    private String ID;
    private Set<String> locations;
    private boolean abTest;

    public Feature(String ID, Set<String> locations, boolean abTest) {
        this.ID = ID;
        this.locations = locations;
        this.abTest = abTest;
    }

    public String getID() {
        return ID;
    }

    public Set<String> getLocations() {
        return locations;
    }

    public boolean isAbTest() {
        return abTest;
    }
}
