
plugins {
    id("java")
}
group = "dev.kingtux"

sourceSets{
    main{
        java{
            srcDir("src")
        }
    }
}

java{
    sourceCompatibility = JavaVersion.VERSION_1_7
    targetCompatibility = JavaVersion.VERSION_1_7
}

tasks{
    jar{
        manifest {
            attributes["Main-Class"] = "dev.kingtux.jur.Main"
        }
    }
}