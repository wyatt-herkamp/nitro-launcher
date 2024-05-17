package dev.kingtux.jur;

public final class Main {
    private static final String[] CHECKED_PROPERTIES = new String[] { "java.version", "java.vendor","java.specification.version"};

    public static void main(String[] args) {
        int returnCode = 0;

        for (String key : CHECKED_PROPERTIES) {
            String property = System.getProperty(key);

            if (property != null) {
                System.out.println(key + "=" + property);
            } else {
                returnCode = 1;
            }
        }

        System.exit(returnCode);
    }
}