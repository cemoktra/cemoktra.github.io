use chrono::NaiveDate;
use leptos::prelude::*;
use thaw::*;

#[derive(Clone)]
struct Job {
    pub company: String,
    pub description: String,
    pub start: NaiveDate,
    pub end: Option<NaiveDate>,
    pub badges: Vec<String>,
}

#[component]
pub fn Cv() -> impl IntoView {
    let (jobs, _) = signal(vec![
        Job {
            company: "Rakuten Luxembourg".into(),
            start: NaiveDate::from_ymd_opt(2025, 1, 1).expect("This is a valid date"),
            end: None,
            badges: vec!["gRPC".into(), "Rust".into(), "Backend".into()],
            description: r#"Finally the intellectual property has moved and the development
               of the platform continues.
               "#
            .into(),
        },
        Job {
            company: "Rakuten Europe Bank".into(),
            start: NaiveDate::from_ymd_opt(2024, 8, 1).expect("This is a valid date"),
            end: Some(NaiveDate::from_ymd_opt(2024, 12, 31).expect("This is a valid date")),
            badges: vec!["Rust".into(), "Backend".into()],
            description: r#"
                End of may 2024 Dock Financial filed for insolvency and Rakuten Europa Bank
                took over the intellectual property and also man power. During those month i
                was consulting the movement and continued to work on the platform.
                "#
            .into(),
        },
        Job {
            company: "Dock Financial GmbH".into(),
            start: NaiveDate::from_ymd_opt(2022, 2, 1).expect("This is a valid date"),
            end: Some(NaiveDate::from_ymd_opt(2024, 7, 31).expect("This is a valid date")),
            badges: vec!["gRPC".into(), "Rust".into(), "Backend".into()],
            description: r#"
                Dock Financial was the successor of Crosscard as the company changed owner and
                business model. Still doing credit card issuing the platform was written from
                scratch in Rust.
                "#
            .into(),
        },
        Job {
            company: "Crosscard GmbH".into(),
            start: NaiveDate::from_ymd_opt(2020, 7, 15).expect("Thisis a valid date"),
            end: Some(NaiveDate::from_ymd_opt(2022, 1, 31).expect("Thisis a valid date")),
            badges: vec![
                "C++".into(),
                "Backend".into(),
                "Python".into(),
                "Rust".into(),
            ],
            description: r#"
                I started as backend engineer for an existing credit card issuing platform. The
                main part was written in C++ with API layers in Python. Closing to the end i
                started writing some services in Rust.
            "#
            .into(),
        },
        Job {
            company: "OSB AG".into(),
            start: NaiveDate::from_ymd_opt(2018, 2, 1).expect("This is a valid date"),
            end: Some(NaiveDate::from_ymd_opt(2020, 7, 14).expect("This is a valid date")),
            badges: vec!["C++".into(), "QML".into(), "Python".into()],
            description: r#"
                Initially i was working for a contractor (EOS GmbH) producing 3D printers. The
                main job was providing interfaces for the printing process and the machine
                interface (QML, touch screen).

                After the contractual work i started learning Rust, did some machine learning
                courses and implemented an internal job crawler using python and React.
            "#
            .into(),
        },
        Job {
            company: "Simi Realtiy Motion Systems".into(),
            start: NaiveDate::from_ymd_opt(2008, 4, 1).expect("This is a valid date"),
            end: Some(NaiveDate::from_ymd_opt(2018, 1, 31).expect("This is a valid date")),
            badges: vec![
                "C++".into(),
                "MFC".into(),
                "COM".into(),
                "Qt".into(),
                "FullStack".into(),
            ],
            description: r#"
                I worked on different motion analysis software solutions written in C/C++
                with frontends  written in MFC and Qt. As a full stack developer i was touching
                all topics from hardware interfaces (camera, force plates) to image and data
                processing and the frontend.
                "#
            .into(),
        },
    ]);

    view! {
        <Flex vertical=true>
            <Persona
                name="Bastian Schubert"
            >
                <PersonaSecondaryText slot>
                    "Senior Software Engineer"
                </PersonaSecondaryText>
            </Persona>

            <For
            each=move || jobs.get()
            key=|job| job.company.clone()
            let(job)
            >
                <Job job/>
            </For>
        </Flex>
    }
}

#[component]
fn Job(job: Job) -> impl IntoView {
    let (date, _) = signal(format!(
        "{} - {}",
        job.start,
        job.end
            .map(|d| d.to_string())
            .unwrap_or_else(|| String::from("now"))
    ));

    let (company, _) = signal(job.company);
    //let (badges, _) = signal(job.badges);

    let badges = job.badges.clone();
    view! {
        <Card>
            <CardHeader>
                <Flex>
                    <Label>{company}</Label>
                    {
                        badges
                            .into_iter()
                            .map(|badge| view! {
                                <Badge appearance=BadgeAppearance::Filled>{badge}</Badge>
                            })
                            .collect::<Vec<_>>()
                    }
                </Flex>
                <CardHeaderDescription slot>
                    <Caption1>{date}</Caption1>
                </CardHeaderDescription>
            </CardHeader>
            <Text>{job.description}</Text>
        </Card>
    }
}
