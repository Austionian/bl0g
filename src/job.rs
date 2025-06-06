#[derive(serde::Serialize, Clone)]
pub struct Job {
    company: &'static str,
    title: &'static str,
    start_year: &'static str,
    end_year: &'static str,
    image: &'static str,
    tech: [&'static str; 5],
    description: &'static str,
    link: &'static str,
}

pub static JOBS: [Job; 3] = [
    Job {
        company: "Northwestern Mutual",
        title: "Software Engineer",
        start_year: "2022",
        end_year: "Present",
        image: "nm.webp",
        tech: ["React", "Redux", "TypeScript", "Express", "Graphql"],
        description: "I work on the client website building new products, \
                supporting legacy apps, and contributing to interal libraries \
                to support easier development. Helped transition the org to \
                Graphql. Lead the team in testing and performance strategies \
                for our React code and subgraph.<br /><br />Was recognized as \
                the individual contributor of Q2 2023 of Northwestern Mutual’s \
                web and mobile engineers for having decreased the entire client \
                website’s LCP by ~500ms.<br /><br />Built a custom SSG framework \
                to be used with vanilla vite configs for prerendering static \
                apps to improve apps' performance in the org's transition to \
                React frontends served from S3.",
        link: "https://northwesternmutual.com",
    },
    Job {
        company: "Oxidized Systems",
        title: "Owner, Software Engineer",
        start_year: "2021",
        end_year: "Present",
        image: "os.png",
        tech: ["Rust", "Actix", "TypeScript", "Qwik", "Postgres"],
        description: "Previous work includes converting a headless JS app into a \
                Qwik/ Qwik-City (TypeScript) frontend and an Actix (Rust) backend \
                for web and native mobile clients to consume.<br /><br />The \
                frontend scores <image alt=\"100 performance score\" src=\"assets/images/100.png\" style=\"height: 2rem; display: inline\"/> \
                across every Lighthouse metric on <span style=\"font-weight: bold\">both</span> web and mobile. \
                The backend test coverage is over 65%.",
        link: "https://oxidized.systems",
    },
    Job {
        company: "Catholic Charities of St. Paul and Minneapolis",
        title: "Software Developer/ Administrator",
        start_year: "2017",
        end_year: "2022",
        image: "ccspm.webp",
        tech: ["Python", "Django", "Flask", "Postgres", "HTMX"],
        description: "I built and maintained internal web apps (Flask and \
                Django) for critical agency wide functions, e.g. critical \
                incidents, employee management. Also administrated and built out \
                instances of SharePoint and EHR (Credible).",
        link: "https://cctwincities.org",
    },
];
