use super::models::Report;

pub struct ReportsService {
    reports: Vec<Report>,
}

impl ReportsService {
    pub fn new() -> Self {
        ReportsService {
            reports: vec![
                Report {
                    id: "abc-123".to_string(),
                    title: "dummy report".to_string(),
                    owner_id: "04781408-1081-706c-c3ac-3c618d5a379a".to_string(),
                    s3_key: "dummy/key/file.pdf".to_string(),
                },
                Report {
                    id: "abc-124".to_string(),
                    title: "dummy report 2".to_string(),
                    owner_id: "123".to_string(),
                    s3_key: "dummy/key/file.pdf".to_string(),
                },
                Report {
                    id: "abc-125".to_string(),
                    title: "dummy report 3".to_string(),
                    owner_id: "321".to_string(),
                    s3_key: "dummy/key/file.pdf".to_string(),
                },
            ],
        }
    }

    pub fn get_report(&self, report_id: String) -> Option<Report> {
        self.reports
            .clone()
            .into_iter()
            .find(|report| report.id == report_id)
    }
}
