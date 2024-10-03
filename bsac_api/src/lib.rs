pub mod request;
pub mod model;
use std::sync::OnceLock;
use std::borrow::Cow;
use httpclient::Client;
static SHARED_HTTPCLIENT: OnceLock<Client> = OnceLock::new();
pub fn default_http_client() -> Client {
    let url = "https://bsac.hlofiys.xyz";
    Client::new()
        .base_url(&url)
}
/// Use this method if you want to add custom middleware to the httpclient.
/// It must be called before any requests are made, otherwise it will have no effect.
/// Example usage:
///
/// ```
/// init_http_client(default_http_client()
///     .with_middleware(..)
/// );
/// ```
pub fn init_http_client(init: Client) {
    let _ = SHARED_HTTPCLIENT.set(init);
}
fn shared_http_client() -> Cow<'static, Client> {
    Cow::Borrowed(SHARED_HTTPCLIENT.get_or_init(default_http_client))
}
#[derive(Clone)]
pub struct FluentRequest<'a, T> {
    pub(crate) client: &'a BsacApiClient,
    pub params: T,
}
pub struct BsacApiClient {
    client: Cow<'static, Client>,
}
impl BsacApiClient {
    pub fn from_env() -> Self {
        Self {
            client: shared_http_client(),
        }
    }
    pub fn new() -> Self {
        Self {
            client: shared_http_client(),
        }
    }
}
impl BsacApiClient {
    ///Gets all exams for group
    pub fn get_group_exams(
        &self,
        group_id: i64,
    ) -> FluentRequest<'_, request::GetGroupExamsRequest> {
        FluentRequest {
            client: self,
            params: request::GetGroupExamsRequest {
                group_id,
            },
        }
    }
    ///Gets all groups
    pub fn get_groups(&self) -> FluentRequest<'_, request::GetGroupsRequest> {
        FluentRequest {
            client: self,
            params: request::GetGroupsRequest {},
        }
    }
    ///Gets all ktps
    pub fn get_ktps(
        &self,
        lesson_id: i64,
    ) -> FluentRequest<'_, request::GetKtpsRequest> {
        FluentRequest {
            client: self,
            params: request::GetKtpsRequest {
                lesson_id,
            },
        }
    }
    ///Gets all lessons
    pub fn get_lessons(&self) -> FluentRequest<'_, request::GetLessonsRequest> {
        FluentRequest {
            client: self,
            params: request::GetLessonsRequest {},
        }
    }
    ///Gets all practices for group
    pub fn get_practices(&self) -> FluentRequest<'_, request::GetPracticesRequest> {
        FluentRequest {
            client: self,
            params: request::GetPracticesRequest {
                group_id: None,
            },
        }
    }
    ///Gets all schedule additions for group
    pub fn get_schedule_additions(
        &self,
        group_id: i64,
    ) -> FluentRequest<'_, request::GetScheduleAdditionsRequest> {
        FluentRequest {
            client: self,
            params: request::GetScheduleAdditionsRequest {
                group_id,
            },
        }
    }
    ///Gets all schedule moves for group
    pub fn get_schedule_moves(
        &self,
        group_id: i64,
    ) -> FluentRequest<'_, request::GetScheduleMovesRequest> {
        FluentRequest {
            client: self,
            params: request::GetScheduleMovesRequest {
                group_id,
            },
        }
    }
    ///Gets current week
    pub fn get_week(&self) -> FluentRequest<'_, request::GetWeekRequest> {
        FluentRequest {
            client: self,
            params: request::GetWeekRequest {},
        }
    }
    ///Gets schedule for users subscribed groups
    pub fn get_schedule_for_groups(
        &self,
    ) -> FluentRequest<'_, request::GetScheduleForGroupsRequest> {
        FluentRequest {
            client: self,
            params: request::GetScheduleForGroupsRequest {
                groups_ids: None,
            },
        }
    }
    ///Gets schedule for user subscribed teachers
    pub fn get_schedule_for_teachers(
        &self,
    ) -> FluentRequest<'_, request::GetScheduleForTeachersRequest> {
        FluentRequest {
            client: self,
            params: request::GetScheduleForTeachersRequest {
                teachers_ids: None,
            },
        }
    }
    ///Get works schedule
    pub fn get_works_schedule(
        &self,
    ) -> FluentRequest<'_, request::GetWorksScheduleRequest> {
        FluentRequest {
            client: self,
            params: request::GetWorksScheduleRequest {
                group_id: None,
                lesson_id: None,
                teacher_id: None,
            },
        }
    }
    ///Gets the schedule actuality
    pub fn get_schedule_actuality(
        &self,
    ) -> FluentRequest<'_, request::GetScheduleActualityRequest> {
        FluentRequest {
            client: self,
            params: request::GetScheduleActualityRequest {
                last_interact: None,
            },
        }
    }
    ///Get group schedule for date
    pub fn get_group_schedule_for_date(
        &self,
        group_id: i64,
    ) -> FluentRequest<'_, request::GetGroupScheduleForDateRequest> {
        FluentRequest {
            client: self,
            params: request::GetGroupScheduleForDateRequest {
                dates: None,
                group_id,
            },
        }
    }
    ///Get teacher schedule for date
    pub fn get_teacher_schedule_for_date(
        &self,
        teacher_id: i64,
    ) -> FluentRequest<'_, request::GetTeacherScheduleForDateRequest> {
        FluentRequest {
            client: self,
            params: request::GetTeacherScheduleForDateRequest {
                dates: None,
                teacher_id,
            },
        }
    }
    ///Gets all teachers
    pub fn get_teachers(&self) -> FluentRequest<'_, request::GetTeachersRequest> {
        FluentRequest {
            client: self,
            params: request::GetTeachersRequest {},
        }
    }
    ///Gets all works for group
    pub fn get_works(
        &self,
        group_id: i64,
    ) -> FluentRequest<'_, request::GetWorksRequest> {
        FluentRequest {
            client: self,
            params: request::GetWorksRequest {
                group_id,
            },
        }
    }
}
