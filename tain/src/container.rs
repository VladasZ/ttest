use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockerContainer {
    pub command:       String,
    pub created_at:    String,
    #[serde(rename = "ID")]
    pub id:            String,
    pub image:         String,
    pub labels:        String,
    pub local_volumes: String,
    pub mounts:        String,
    pub names:         String,
    pub networks:      String,
    pub ports:         String,
    pub running_for:   String,
    pub size:          String,
    pub state:         String,
    pub status:        String,
}
